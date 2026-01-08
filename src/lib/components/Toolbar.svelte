<script lang="ts">
    import { open, save } from '@tauri-apps/plugin-dialog';
    import { editor, fileName } from '$lib/stores/editor';
    import { templateStore } from '$lib/stores/template';
    import { settings } from '$lib/stores/settings';

    let {
        onopen,
        onsave,
        onexportxml,
    }: {
        onopen?: () => void;  // Parent handles opening (shows dialog, loads project)
        onsave?: () => void;  // Parent handles saving (shows dialog if needed, saves project)
        onexportxml?: () => void;  // Parent handles XML export
    } = $props();

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
        <button class="btn btn-primary btn-sm" onclick={onopen} title="Open project (Ctrl+O)">Open</button>
        <button class="btn btn-primary btn-sm" onclick={onsave} title="Save project (Ctrl+S)">Save</button>
        <button class="btn btn-ghost btn-sm" onclick={onexportxml} title="Export TEI-XML to separate file">Export XML</button>
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
