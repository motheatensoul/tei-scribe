import {
    type Annotation,
    type AnnotationSet,
    type AnnotationType,
    type AnnotationTarget,
    type AnnotationValue,
    type AnnotationMetadata,
    createEmptyAnnotationSet,
    createLemmaAnnotation,
    getAnnotationsForWord,
    getAnnotationsByType,
    getLemmaMap,
    targetIncludesWord,
} from "$lib/types/annotations";

import { Tag, BookOpen, StickyNote, PenTool, Link, Info } from "@lucide/svelte";

// Export types
export type { 
    Annotation, 
    AnnotationSet, 
    AnnotationType, 
    AnnotationTarget, 
    AnnotationValue, 
    AnnotationMetadata 
};

// Export helpers
export { 
    createEmptyAnnotationSet, 
    createLemmaAnnotation, 
    getAnnotationsForWord, 
    getAnnotationsByType, 
    getLemmaMap, 
    targetIncludesWord 
};

/** Get annotation type label */
export function getTypeLabel(type: AnnotationType): string {
    switch (type) {
        case "lemma": return "Lemma";
        case "semantic": return "Semantic";
        case "note": return "Note";
        case "paleographic": return "Paleographic";
        case "syntax": return "Syntax";
        case "reference": return "Reference";
        case "custom": return "Custom";
        default: return "Unknown";
    }
}

/** Get annotation type icon */
export function getTypeIcon(type: AnnotationType) {
    switch (type) {
        case "lemma": return Tag;
        case "semantic": return BookOpen;
        case "note": return StickyNote;
        case "paleographic": return PenTool;
        case "reference": return Link;
        default: return Info;
    }
}

// ============================================================================
// Annotation History (for undo/redo)
// ============================================================================

export interface AnnotationAction {
    type: "add" | "remove" | "update";
    annotation: Annotation;
    previousAnnotation?: Annotation | null;
}

class AnnotationHistory {
    #undoStack = $state<AnnotationAction[]>([]);
    #redoStack = $state<AnnotationAction[]>([]);
    #maxHistory = $state(50);

    get undoStack() { return this.#undoStack; }
    get redoStack() { return this.#redoStack; }
    get canUndo() { return this.#undoStack.length > 0; }
    get canRedo() { return this.#redoStack.length > 0; }

    pushAction(action: AnnotationAction) {
        this.#undoStack.push(action);
        if (this.#undoStack.length > this.#maxHistory) {
            this.#undoStack.shift();
        }
        this.#redoStack = []; // Clear redo stack on new action
    }

    undo(): AnnotationAction | null {
        const action = this.#undoStack.pop();
        if (action) {
            this.#redoStack.push(action);
            return action;
        }
        return null;
    }

    redo(): AnnotationAction | null {
        const action = this.#redoStack.pop();
        if (action) {
            this.#undoStack.push(action);
            return action;
        }
        return null;
    }

    clear() {
        this.#undoStack = [];
        this.#redoStack = [];
    }

    setMaxHistory(max: number) {
        this.#maxHistory = max;
        if (this.#undoStack.length > max) {
            this.#undoStack = this.#undoStack.slice(-max);
        }
    }
}

export const annotationHistory = new AnnotationHistory();

// ============================================================================
// Main Annotation Store
// ============================================================================

export class AnnotationStore {
    #set = $state<AnnotationSet>(createEmptyAnnotationSet());

    get set() { return this.#set; }

    // Computed properties
    get annotations() { return this.#set.annotations; }

    get annotationsByWordIndex() {
        const index = new Map<number, Annotation[]>();
        for (const ann of this.#set.annotations) {
            const wordIndices: number[] = [];
            if (ann.target.type === "word") {
                wordIndices.push(ann.target.wordIndex);
            } else if (ann.target.type === "span") {
                for (let i = ann.target.startWord; i <= ann.target.endWord; i++) {
                    wordIndices.push(i);
                }
            }
            for (const wordIndex of wordIndices) {
                const existing = index.get(wordIndex) || [];
                existing.push(ann);
                index.set(wordIndex, existing);
            }
        }
        return index;
    }

    get confirmedLemmaIndices() {
        const confirmed = new Set<number>();
        for (const ann of this.#set.annotations) {
            if (ann.type === "lemma" && ann.target.type === "word") {
                confirmed.add(ann.target.wordIndex);
            }
        }
        return confirmed;
    }

    get counts() {
        const counts: Record<AnnotationType, number> = {
            lemma: 0,
            semantic: 0,
            note: 0,
            paleographic: 0,
            syntax: 0,
            reference: 0,
            custom: 0,
        };
        for (const ann of this.#set.annotations) {
            counts[ann.type]++;
        }
        return counts;
    }

    get lemmaMappings() {
        const mappings: Record<number, { lemma: string; msa: string; normalized?: string }> = {};
        for (const ann of this.#set.annotations) {
            if (ann.type === "lemma" && ann.target.type === "word" && ann.value.kind === "lemma") {
                mappings[ann.target.wordIndex] = {
                    lemma: ann.value.lemma,
                    msa: ann.value.msa,
                    normalized: ann.value.normalized,
                };
            }
        }
        return mappings;
    }

    add(annotation: Annotation, recordHistory = true) {
        const existingIndex = this.#set.annotations.findIndex(a => a.id === annotation.id);
        let previousAnnotation: Annotation | null = null;
        
        if (existingIndex >= 0) {
            previousAnnotation = this.#set.annotations[existingIndex];
            this.#set.annotations[existingIndex] = annotation;
        } else {
            this.#set.annotations.push(annotation);
        }

        if (recordHistory) {
            annotationHistory.pushAction({
                type: existingIndex >= 0 ? "update" : "add",
                annotation,
                previousAnnotation,
            });
        }
    }

    remove(id: string, recordHistory = true) {
        const index = this.#set.annotations.findIndex(a => a.id === id);
        if (index < 0) return;

        const annotation = this.#set.annotations[index];
        this.#set.annotations.splice(index, 1);

        if (recordHistory) {
            annotationHistory.pushAction({
                type: "remove",
                annotation,
                previousAnnotation: annotation,
            });
        }
    }

    loadSet(annotationSet: AnnotationSet) {
        this.#set = annotationSet;
        annotationHistory.clear();
    }

    clear() {
        this.#set = createEmptyAnnotationSet();
        annotationHistory.clear();
    }

    // Helper methods
    getForWord(wordIndex: number): Annotation[] {
        return this.annotationsByWordIndex.get(wordIndex) || [];
    }

    isLemmaConfirmed(wordIndex: number): boolean {
        return this.confirmedLemmaIndices.has(wordIndex);
    }

    getLemmaMapping(wordIndex: number) {
        return this.lemmaMappings[wordIndex];
    }

    confirmLemma(wordIndex: number, lemma: string, msa: string, normalized?: string) {
        const annotation = createLemmaAnnotation(wordIndex, lemma, msa, normalized);
        this.add(annotation);
    }

    unconfirmLemma(wordIndex: number) {
        const existing = this.#set.annotations.find(
            a => a.type === "lemma" && a.target.type === "word" && a.target.wordIndex === wordIndex
        );
        if (existing) {
            this.remove(existing.id);
        }
    }

    loadLegacyConfirmations(confirmations: Record<number, { lemma: string; msa: string; normalized?: string }>) {
        if (!confirmations) return;
        for (const [indexStr, mapping] of Object.entries(confirmations)) {
            const wordIndex = parseInt(indexStr);
            if (!isNaN(wordIndex)) {
                this.confirmLemma(wordIndex, mapping.lemma, mapping.msa, mapping.normalized);
            }
        }
    }
}

export const annotationStore = new AnnotationStore();
