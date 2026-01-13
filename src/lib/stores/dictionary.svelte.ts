import type { OnpEntry, InflectedForm } from '$lib/tauri';

// Re-export annotation store and related items for backward compatibility
export {
    annotationStore,
    annotationStore as sessionLemmaStore,
    annotationHistory,
    annotationHistory as lemmatizationHistory,
} from './annotations.svelte';

export type { Annotation, AnnotationValue } from './annotations.svelte';

class DictionaryStore {
    #state = $state({
        loaded: false,
        loading: false,
        headwordCount: 0,
        error: null as string | null,
    });

    get loaded() { return this.#state.loaded; }
    get loading() { return this.#state.loading; }
    get headwordCount() { return this.#state.headwordCount; }
    get error() { return this.#state.error; }

    setLoading() {
        this.#state.loading = true;
        this.#state.error = null;
    }

    setLoaded(count: number) {
        this.#state.loaded = true;
        this.#state.loading = false;
        this.#state.headwordCount = count;
        this.#state.error = null;
    }

    setError(error: string) {
        this.#state.loaded = false;
        this.#state.loading = false;
        this.#state.error = error;
    }

    reset() {
        this.#state.loaded = false;
        this.#state.loading = false;
        this.#state.headwordCount = 0;
        this.#state.error = null;
    }
}

export const dictionaryStore = new DictionaryStore();

class LemmaSearchStore {
    #state = $state({
        query: '',
        results: [] as OnpEntry[],
        loading: false,
    });

    get query() { return this.#state.query; }
    get results() { return this.#state.results; }
    get loading() { return this.#state.loading; }

    setQuery(query: string) {
        this.#state.query = query;
    }

    setResults(results: OnpEntry[]) {
        this.#state.results = results;
        this.#state.loading = false;
    }

    setLoading() {
        this.#state.loading = true;
    }

    clear() {
        this.#state.query = '';
        this.#state.results = [];
        this.#state.loading = false;
    }
}

export const lemmaSearchStore = new LemmaSearchStore();

class InflectionStore {
    #mappings = $state<Record<string, InflectedForm[]>>({});
    #loaded = $state(false);

    get mappings() { return this.#mappings; }
    get loaded() { return this.#loaded; }

    setMappings(mappings: Record<string, InflectedForm[]>) {
        this.#mappings = mappings;
        this.#loaded = true;
    }

    addMapping(wordform: string, inflection: InflectedForm) {
        const key = wordform.toLowerCase();
        const existing = this.#mappings[key] || [];
        const alreadyExists = existing.some(
            (f) => f.onp_id === inflection.onp_id && f.analysis === inflection.analysis
        );
        if (alreadyExists) return;
        
        this.#mappings[key] = [...existing, inflection];
    }

    removeMapping(wordform: string, onpId: string, analysis: string) {
        const key = wordform.toLowerCase();
        const existing = this.#mappings[key] || [];
        const filtered = existing.filter(
            (f) => !(f.onp_id === onpId && f.analysis === analysis)
        );
        
        if (filtered.length === 0) {
            delete this.#mappings[key];
        } else {
            this.#mappings[key] = filtered;
        }
    }

    clear() {
        this.#mappings = {};
        this.#loaded = false;
    }

    // Direct methods instead of derived stores for simpler usage in components
    hasInflection(wordform: string): boolean {
        return wordform.toLowerCase() in this.#mappings;
    }

    getInflections(wordform: string): InflectedForm[] {
        return this.#mappings[wordform.toLowerCase()] || [];
    }

    get knownInflectionForms() {
        return new Set(Object.keys(this.#mappings));
    }
}

export const inflectionStore = new InflectionStore();
