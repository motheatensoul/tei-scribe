<script lang="ts">
    import { open, save } from '@tauri-apps/plugin-dialog';
    import { editor, fileName } from '$lib/stores/editor';
    import { templateStore } from '$lib/stores/template';
    import { settings } from '$lib/stores/settings';
    import { openFile, saveFile, exportTei, listTemplates, compileDsl } from '$lib/tauri';

    let {
        onopen,
        onsave,
        onexport,
    }: {
        onopen?: (content: string) => void;
        onsave?: () => void;
        onexport?: () => void;
    } = $props();

    async function handleOpen() {
        const path = await open({
            filters: [{ name: 'TEI Scribe', extensions: ['teis', 'txt'] }],
        });
        if (path) {
            const file = await openFile(path as string);
            editor.setFile(file.path, file.content);
            onopen?.(file.content);
        }
    }

    async function handleSave() {
        let path = $editor.filePath;
        if (!path) {
            path = await save({
                filters: [{ name: 'TEI Scribe', extensions: ['teis'] }],
            });
        }
        if (path) {
            await saveFile(path, $editor.content);
            editor.setFile(path, $editor.content);
            onsave?.();
        }
    }

    async function handleExport() {
        const template = $templateStore.active;
        if (!template) {
            alert('Please select a template first');
            return;
        }

        const path = await save({
            filters: [{ name: 'TEI-XML', extensions: ['xml'] }],
        });

        if (path) {
            const teiContent = await compileDsl(
                $editor.content,
                template.header,
                template.footer
            );
            await exportTei(path, teiContent);
            onexport?.();
        }
    }

    async function handleTemplateChange(e: Event) {
        const select = e.target as HTMLSelectElement;
        const templates = $templateStore.templates;
        const template = templates.find((t) => t.id === select.value);
        if (template) {
            templateStore.setActive(template);
            settings.update({ activeTemplateId: template.id });
        }
    }
</script>

<div class="navbar bg-neutral text-neutral-content px-4 min-h-12">
    <div class="flex gap-2">
        <button class="btn btn-primary btn-sm" onclick={handleOpen} title="Open file (Ctrl+O)">Open</button>
        <button class="btn btn-primary btn-sm" onclick={handleSave} title="Save file (Ctrl+S)">Save</button>
        <button class="btn btn-primary btn-sm" onclick={handleExport} title="Export to TEI-XML">Export</button>
    </div>

    <div class="flex items-center gap-2 ml-4">
        <span class="text-sm">Template:</span>
        <select
            class="select select-sm select-bordered bg-neutral-focus"
            onchange={handleTemplateChange}
            value={$templateStore.active?.id ?? ''}
        >
            {#each $templateStore.templates as template}
                <option value={template.id}>{template.name}</option>
            {/each}
        </select>
    </div>

    <div class="ml-auto flex items-center gap-1">
        <span class="text-sm opacity-70">{$fileName}</span>
        {#if $editor.isDirty}
            <span class="text-warning font-bold">*</span>
        {/if}
    </div>
</div>
