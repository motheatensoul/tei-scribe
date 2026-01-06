<script lang="ts">
    import { templateStore, type Template } from '$lib/stores/template';
    import { saveTemplate } from '$lib/tauri';

    let { onclose }: { onclose?: () => void } = $props();

    let editingTemplate: Template | null = $state(null);
    let isCreating = $state(false);

    function handleEdit(template: Template) {
        editingTemplate = { ...template };
        isCreating = false;
    }

    function handleCreate() {
        editingTemplate = {
            id: '',
            name: '',
            description: '',
            header: '',
            footer: '',
            wordWrap: false,
            autoLineNumbers: false,
            multiLevel: false,
        };
        isCreating = true;
    }

    async function handleSave() {
        if (!editingTemplate) return;

        if (isCreating && !editingTemplate.id) {
            editingTemplate.id = editingTemplate.name
                .toLowerCase()
                .replace(/\s+/g, '-')
                .replace(/[^a-z0-9-]/g, '');
        }

        await saveTemplate(editingTemplate);
        editingTemplate = null;
    }

    function handleCancel() {
        editingTemplate = null;
    }
</script>

<div>
    <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-bold">Template Manager</h2>
        <button class="btn btn-ghost btn-sm" onclick={onclose}>×</button>
    </div>

    {#if editingTemplate}
        <div class="flex flex-col gap-4">
            <label class="form-control">
                <div class="label"><span class="label-text font-medium">Name</span></div>
                <input type="text" class="input input-bordered" bind:value={editingTemplate.name} />
            </label>
            <label class="form-control">
                <div class="label"><span class="label-text font-medium">Description</span></div>
                <input type="text" class="input input-bordered" bind:value={editingTemplate.description} />
            </label>
            <label class="form-control">
                <div class="label"><span class="label-text font-medium">Header (TEI-XML before content)</span></div>
                <textarea class="textarea textarea-bordered font-mono text-sm h-40" bind:value={editingTemplate.header}></textarea>
            </label>
            <label class="form-control">
                <div class="label"><span class="label-text font-medium">Footer (TEI-XML after content)</span></div>
                <textarea class="textarea textarea-bordered font-mono text-sm h-24" bind:value={editingTemplate.footer}></textarea>
            </label>
            <label class="label cursor-pointer justify-start gap-2">
                <input type="checkbox" class="checkbox checkbox-sm" bind:checked={editingTemplate.wordWrap} />
                <span class="label-text">Enable word wrapping (&lt;w&gt; tags)</span>
            </label>
            <label class="label cursor-pointer justify-start gap-2">
                <input type="checkbox" class="checkbox checkbox-sm" bind:checked={editingTemplate.autoLineNumbers} />
                <span class="label-text">Automatic line numbering (&lt;lb n="..."&gt;)</span>
            </label>
            <label class="label cursor-pointer justify-start gap-2">
                <input type="checkbox" class="checkbox checkbox-sm" bind:checked={editingTemplate.multiLevel} />
                <span class="label-text">Multi-level output (MENOTA me:facs/me:dipl/me:norm)</span>
            </label>
            <div class="flex justify-end gap-2 mt-2">
                <button class="btn btn-ghost" onclick={handleCancel}>Cancel</button>
                <button class="btn btn-primary" onclick={handleSave}>Save</button>
            </div>
        </div>
    {:else}
        <div class="flex flex-col gap-2">
            {#each $templateStore.templates as template}
                <div class="flex justify-between items-center p-3 bg-base-200 rounded-lg">
                    <div class="flex flex-col gap-1">
                        <strong>{template.name}</strong>
                        <span class="text-sm opacity-70">{template.description}</span>
                    </div>
                    <button class="btn btn-ghost btn-sm" onclick={() => handleEdit(template)}>Edit</button>
                </div>
            {/each}
        </div>
        <button class="btn btn-outline w-full mt-4" onclick={handleCreate}>+ Create Template</button>
    {/if}
</div>
