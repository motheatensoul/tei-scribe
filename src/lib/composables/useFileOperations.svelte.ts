import { open, save } from "@tauri-apps/plugin-dialog";
import { editor } from "$lib/stores/editor.svelte";
import { templateStore } from "$lib/stores/template.svelte";
import { annotationStore, annotationHistory } from "$lib/stores/annotations.svelte";
import { metadataStore } from "$lib/stores/metadata.svelte";
import { errorStore } from "$lib/stores/errors.svelte";
import { inflectionStore, dictionaryStore } from "$lib/stores/dictionary.svelte";
import { entityStore } from "$lib/stores/entities.svelte";
import {
    openProject,
    saveProject,
    openFile,
    importFile,
    exportTei,
    exportHtml,
    exportInflections
} from "$lib/tauri";
import { generateStandaloneHtml } from "$lib/utils/htmlExport";
import { printToPdf } from "$lib/utils/pdfExport";
import { applyXsltTransform as sharedApplyXsltTransform } from "$lib/utils/xslt";
import { tick } from "svelte";

export interface FileOperationsOptions {
    doCompile: (content: string) => Promise<void>;
    getPreviewContent: () => string;
    setIsImporting: (val: boolean) => void;
    getEditorComponent: () => any;
}

export function createFileOperations(options: FileOperationsOptions) {
    const { doCompile, getPreviewContent, setIsImporting, getEditorComponent } = options;

    async function applyXsltTransform(xmlContent: string): Promise<string> {
        const response = await fetch("/xsl/simple.xsl");
        if (!response.ok) throw new Error(`Failed to load stylesheet: ${response.statusText}`);
        const xslText = await response.text();
        return sharedApplyXsltTransform(xmlContent, xslText, entityStore.entities);
    }

    async function handleOpenProject() {
        const path = await open({
            filters: [
                { name: "TEI Scribe Project", extensions: ["teis"] },
                { name: "DSL Source", extensions: ["dsl", "txt"] },
            ],
        });
        if (!path) return;
        const pathStr = path as string;
        try {
            if (pathStr.endsWith(".teis")) {
                const project = await openProject(pathStr);
                editor.setFile(pathStr, project.source);
                // We need to set content on the editor component too if it's not bound two-way or if we need to force update
                const editorComponent = getEditorComponent();
                if (editorComponent && editorComponent.setContent) {
                    editorComponent.setContent(project.source);
                }
                
                annotationHistory.clear();
                if (project.annotations) {
                    annotationStore.loadSet(project.annotations);
                } else {
                    annotationStore.loadLegacyConfirmations(project.confirmations);
                }
                
                const templates = templateStore.templates;
                const template = templates.find((t) => t.id === project.manifest.template_id);
                if (template) templateStore.setActive(template);
                
                if (project.metadata) {
                    metadataStore.setMetadata(project.metadata);
                } else {
                    metadataStore.resetMetadata();
                }
                
                // Trigger compile
                await doCompile(project.source);
                errorStore.info("Project", `Opened project from ${pathStr}`);
            } else {
                const file = await openFile(pathStr);
                editor.setFile(file.path, file.content);
                const editorComponent = getEditorComponent();
                if (editorComponent && editorComponent.setContent) {
                    editorComponent.setContent(file.content);
                }
                
                annotationHistory.clear();
                annotationStore.clear();
                
                await doCompile(file.content);
            }
        } catch (e) {
            errorStore.error("Open", `Failed to open: ${e}`);
        }
    }

    async function handleSaveProject(silent = false) {
        const template = templateStore.active;
        if (!template) {
            if (!silent) errorStore.warning("Save", "Please select a template before saving");
            return;
        }
        let path = editor.filePath;
        if (!path || !path.endsWith(".teis")) {
            if (silent) return; // Don't prompt for path on auto-save
            path = await save({
                filters: [{ name: "TEI Scribe Project", extensions: ["teis"] }],
                defaultPath: path ? path.replace(/\.[^.]+$/, ".teis") : undefined,
            });
        }
        if (!path) return;
        try {
            // Ensure fresh compile
            await doCompile(editor.content);
            
            const confirmationsJson = JSON.stringify(annotationStore.lemmaMappings);
            const annotationsJson = JSON.stringify(annotationStore.set);
            const metadataJson = JSON.stringify(metadataStore.metadata);
            
            await saveProject(
                path,
                editor.content,
                getPreviewContent(),
                confirmationsJson,
                template.id,
                metadataJson,
                annotationsJson,
            );
            editor.setFile(path, editor.content);
            if (!silent) errorStore.info("Project", `Saved project to ${path}`);
        } catch (e) {
            // Always show errors
            errorStore.error("Save", `Failed to save project: ${e}`);
        }
    }

    async function handleExportXml() {
        const template = templateStore.active;
        if (!template) return;
        const path = await save({
            filters: [{ name: "TEI-XML", extensions: ["xml"] }],
            defaultPath: editor.filePath ? editor.filePath.replace(/\.[^.]+$/, ".xml") : undefined,
        });
        if (!path) return;
        try {
            await doCompile(editor.content);
            await exportTei(path, getPreviewContent());
        } catch (e) {
            errorStore.error("Export", `Failed to export: ${e}`);
        }
    }

    async function handleExportDictionary() {
        const path = await save({
            filters: [{ name: "JSON", extensions: ["json"] }],
            defaultPath: "inflections.json",
        });
        if (!path) return;
        try {
            await exportInflections(path);
        } catch (e) {
            errorStore.error("Export", `Failed to export dictionary: ${e}`);
        }
    }

    async function handleExportHtml() {
        const template = templateStore.active;
        if (!template) return;
        const path = await save({
            filters: [{ name: "HTML", extensions: ["html", "htm"] }],
            defaultPath: editor.filePath ? editor.filePath.replace(/\.[^.]+$/, ".html") : undefined,
        });
        if (!path) return;
        try {
            await doCompile(editor.content);
            const htmlBody = await applyXsltTransform(getPreviewContent());
            const fileName = editor.filePath ? editor.filePath.split("/").pop()?.replace(/\.[^.]+$/, "") : "export";
            const fullHtml = generateStandaloneHtml(htmlBody, { title: fileName ?? "export" });
            await exportHtml(path, fullHtml);
        } catch (e) {
            errorStore.error("Export", `Failed to export HTML: ${e}`);
        }
    }

    async function handleExportPdf() {
        const template = templateStore.active;
        if (!template) return;
        try {
            await doCompile(editor.content);
            const htmlBody = await applyXsltTransform(getPreviewContent());
            const fileName = editor.filePath ? editor.filePath.split("/").pop()?.replace(/\.[^.]+$/, "") : "export";
            const fullHtml = generateStandaloneHtml(htmlBody, { title: fileName ?? "export", pageBreakStyle: "print-break" });
            await printToPdf(fullHtml, { pageSize: "A4" });
        } catch (e) {
            errorStore.error("Export", `Failed to prepare PDF: ${e}`);
        }
    }

    async function handleImport() {
        const path = await open({
            filters: [
                { name: "All Supported Formats", extensions: ["xml", "tei", "txt"] },
            ],
        });
        if (!path) return;
        const pathStr = path as string;
        
        setIsImporting(true);
        await tick();
        
        try {
            const result = await importFile(pathStr);
            annotationHistory.clear();
            annotationStore.clear();
            
            // Compile just to get preview, we also set editor content
            editor.setFile(null, result.dsl);
            const editorComponent = getEditorComponent();
            if (editorComponent && editorComponent.setContent) {
                editorComponent.setContent(result.dsl);
            }
            
            if (result.metadata) {
                metadataStore.setMetadata(result.metadata);
            }
            
            await doCompile(result.dsl);
            
        } catch (e) {
            errorStore.error("Import", `Failed to import: ${e}`);
        } finally {
            await new Promise((resolve) => setTimeout(resolve, 500));
            setIsImporting(false);
        }
    }

    return {
        handleOpenProject,
        handleSaveProject,
        handleExportXml,
        handleExportDictionary,
        handleExportHtml,
        handleExportPdf,
        handleImport
    };
}
