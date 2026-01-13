import type { Entity, EntityMap } from '$lib/types/entities';

export type { Entity, EntityMap };

class EntityStore {
    #state = $state<{
        builtinEntities: EntityMap;
        customEntities: EntityMap;
        baseMappings: Record<string, string>;
        customMappings: Record<string, string>;
        loaded: boolean;
        error: string | null;
    }>({
        builtinEntities: {},
        customEntities: {},
        baseMappings: {},
        customMappings: {},
        loaded: false,
        error: null,
    });

    get builtinEntities() { return this.#state.builtinEntities; }
    get customEntities() { return this.#state.customEntities; }
    get baseMappings() { return this.#state.baseMappings; }
    get customMappings() { return this.#state.customMappings; }
    get loaded() { return this.#state.loaded; }
    get error() { return this.#state.error; }

    // Combined entities
    get entities() {
        return { ...this.#state.builtinEntities, ...this.#state.customEntities };
    }

    // Derived entity names
    get entityNames() {
        return Object.keys(this.entities).sort();
    }

    setBuiltinEntities(entities: EntityMap) {
        this.#state.builtinEntities = entities;
        this.#state.loaded = true;
        this.#state.error = null;
    }

    setCustomEntities(customEntities: EntityMap) {
        this.#state.customEntities = customEntities;
    }

    addCustomEntity(name: string, entity: Entity) {
        this.#state.customEntities[name] = entity;
    }

    removeCustomEntity(name: string) {
        delete this.#state.customEntities[name];
    }

    setError(error: string) {
        this.#state.error = error;
        this.#state.loaded = false;
    }

    setBaseMappings(mappings: Record<string, string>) {
        this.#state.baseMappings = mappings;
    }

    setCustomMappings(mappings: Record<string, string>) {
        this.#state.customMappings = mappings;
    }

    setCustomMapping(entity: string, translation: string) {
        this.#state.customMappings[entity] = translation;
    }

    removeCustomMapping(entity: string) {
        delete this.#state.customMappings[entity];
    }

    reset() {
        this.#state.builtinEntities = {};
        this.#state.customEntities = {};
        this.#state.baseMappings = {};
        this.#state.customMappings = {};
        this.#state.loaded = false;
        this.#state.error = null;
    }

    hasEntity(name: string): boolean {
        return name in this.entities;
    }

    getEntityChar(name: string): string | null {
        return this.entities[name]?.char ?? null;
    }
}

export const entityStore = new EntityStore();
