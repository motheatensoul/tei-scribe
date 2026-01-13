import { onMount } from "svelte";

export interface ShortcutOptions {
    fileOps: {
        handleSaveProject: () => Promise<void>;
        handleOpenProject: () => Promise<void>;
    };
    undoRedoOps: {
        handleLemmaUndo: () => void;
        handleLemmaRedo: () => void;
    };
    uiState: {
        closeAllModals: () => void;
        toggleHelp: () => void;
    };
}

export function createShortcuts(options: ShortcutOptions) {
    const { fileOps, undoRedoOps, uiState } = options;

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Escape") {
            uiState.closeAllModals();
            return;
        }
        if (event.key === "F1") {
            event.preventDefault();
            uiState.toggleHelp();
            return;
        }
        if (event.ctrlKey || event.metaKey) {
            if (event.key === "s") {
                event.preventDefault();
                fileOps.handleSaveProject();
            } else if (event.key === "o") {
                event.preventDefault();
                fileOps.handleOpenProject();
            } else if (event.key === "?" || event.key === "/") {
                event.preventDefault();
                uiState.toggleHelp();
            } else if (event.shiftKey && (event.key === "z" || event.key === "Z")) {
                event.preventDefault();
                undoRedoOps.handleLemmaUndo();
            } else if (event.shiftKey && (event.key === "y" || event.key === "Y")) {
                event.preventDefault();
                undoRedoOps.handleLemmaRedo();
            }
        }
    }

    return {
        handleKeydown
    };
}
