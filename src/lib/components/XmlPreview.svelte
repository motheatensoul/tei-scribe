<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { EditorView } from "codemirror";
    import { EditorState } from "@codemirror/state";
    import { xml } from "@codemirror/lang-xml";
    import { linter, type Diagnostic } from "@codemirror/lint";
    import { validationStore } from "$lib/stores/validation.svelte";
    import { daisyExtensions } from "$lib/editor/theme";

    let {
        content = "",
        onwordclick,
    }: {
        content?: string;
        onwordclick?: (
            facsimile: string,
            diplomatic: string,
            wordIndex: number,
            element: HTMLElement,
        ) => void;
    } = $props();

    let container: HTMLDivElement;
    let view: EditorView;

    // Define a linter source that reads from the validation store
    const xmlLinter = linter((view) => {
        if (!validationStore.lastResult || validationStore.lastResult.valid) return [];
        
        const diagnostics: Diagnostic[] = [];
        for (const err of validationStore.lastResult.errors) {
            // Error lines are 1-based, CodeMirror lines are 1-based for .line() access
            // But we need to be careful about bounds
            if (err.line && err.line <= view.state.doc.lines) {
                 const lineInfo = view.state.doc.line(err.line);
                 diagnostics.push({
                     from: lineInfo.from,
                     to: lineInfo.to,
                     severity: err.isWarning ? "warning" : "error",
                     message: err.message
                 });
            }
        }
        return diagnostics;
    });

    onMount(() => {
        const startState = EditorState.create({
            doc: content,
            extensions: [
                daisyExtensions,
                xml(),
                xmlLinter,
                EditorView.editable.of(false),
                EditorState.readOnly.of(true),
                EditorView.lineWrapping,
                EditorView.theme({
                    "&": { height: "100%" },
                    ".cm-scroller": { overflow: "auto" },
                }),
            ],
        });

        view = new EditorView({
            state: startState,
            parent: container,
        });
    });

    onDestroy(() => {
        if (view) view.destroy();
    });

    // Update editor content when prop changes
    $effect(() => {
        if (view && content !== view.state.doc.toString()) {
            view.dispatch({
                changes: {
                    from: 0,
                    to: view.state.doc.length,
                    insert: content,
                },
            });
        }
    });

    // Trigger linting update when validation changes
    $effect(() => {
        if (view && validationStore.lastResult) {
            // Dispatch a dummy transaction to trigger linter re-run
            view.dispatch({
                // Just dispatch empty to signal update
            });
        }
    });
</script>

<div class="h-full w-full overflow-hidden flex flex-col bg-base-100">
    <div bind:this={container} class="flex-1 overflow-hidden border border-base-300 rounded"></div>
</div>

<style>
    :global(.cm-editor) {
        height: 100%;
    }
</style>
