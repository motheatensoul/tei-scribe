export interface EditorState {
    content: string;
    filePath: string | null;
    isDirty: boolean;
}

class EditorStore {
    #state = $state<EditorState>({
        content: '',
        filePath: null,
        isDirty: false,
    });

    get content() { return this.#state.content; }
    get filePath() { return this.#state.filePath; }
    get isDirty() { return this.#state.isDirty; }

    // Derived value for the display name of the current file
    get fileName() {
        if (!this.#state.filePath) return 'Untitled';
        return this.#state.filePath.split('/').pop() ?? 'Untitled';
    }

    setContent(content: string) {
        this.#state.content = content;
        this.#state.isDirty = true;
    }

    setFile(filePath: string | null, content: string) {
        this.#state.content = content;
        this.#state.filePath = filePath;
        this.#state.isDirty = false;
    }

    markClean() {
        this.#state.isDirty = false;
    }

    reset() {
        this.#state.content = '';
        this.#state.filePath = null;
        this.#state.isDirty = false;
    }
}

export const editor = new EditorStore();
