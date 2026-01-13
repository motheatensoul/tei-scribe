import type { Metadata } from "$lib/types/metadata";
import { createEmptyMetadata } from "$lib/types/metadata";

class MetadataStore {
    #state = $state<Metadata>(createEmptyMetadata());

    get metadata() { return this.#state; }

    setMetadata(metadata: Metadata) {
        this.#state = metadata;
    }

    updateMetadata(partial: Partial<Metadata>) {
        this.#state = { ...this.#state, ...partial };
    }

    resetMetadata() {
        this.#state = createEmptyMetadata();
    }
}

export const metadataStore = new MetadataStore();
