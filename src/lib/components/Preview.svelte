<script lang="ts">
    let { content = '' }: { content?: string } = $props();
</script>

<div class="w-full h-full flex flex-col bg-base-200">
    <div class="px-4 py-2 bg-base-200 border-b border-base-300 font-medium text-sm">
        <span>TEI-XML Preview</span>
    </div>
    <pre class="flex-1 m-0 p-4 overflow-auto font-mono text-sm leading-relaxed whitespace-pre-wrap break-words"><code>{content}</code></pre>
</div>
