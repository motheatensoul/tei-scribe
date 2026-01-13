import { listSchemas, type ValidationResult, type SchemaInfo } from "../tauri";

export interface ValidationState {
    lastResult: ValidationResult | null;
    availableSchemas: SchemaInfo[];
    loaded: boolean;
    error: string | null;
    selectedSchemaId: string;
    isValidating: boolean;
}

class ValidationStore {
    #state = $state<ValidationState>({
        lastResult: null,
        availableSchemas: [],
        loaded: false,
        error: null,
        selectedSchemaId: "tei-p5",
        isValidating: false,
    });

    get lastResult() { return this.#state.lastResult; }
    get availableSchemas() { return this.#state.availableSchemas; }
    get loaded() { return this.#state.loaded; }
    get error() { return this.#state.error; }
    get selectedSchemaId() { return this.#state.selectedSchemaId; }
    get isValidating() { return this.#state.isValidating; }

    // Derived values
    get validationCounts() {
        return {
            errors: this.#state.lastResult?.errorCount ?? 0,
            warnings: this.#state.lastResult?.warningCount ?? 0,
        };
    }

    setResult(result: ValidationResult | null) {
        this.#state.lastResult = result;
        this.#state.isValidating = false;
        this.#state.error = null;
    }

    setSchemas(schemas: SchemaInfo[]) {
        this.#state.availableSchemas = schemas;
        this.#state.loaded = true;
        this.#state.error = null;
    }

    setError(error: string) {
        this.#state.error = error;
        this.#state.loaded = false;
        this.#state.isValidating = false;
    }

    clear() {
        this.#state.lastResult = null;
        this.#state.isValidating = false;
        this.#state.error = null;
    }

    async loadSchemas() {
        try {
            const schemas = await listSchemas();
            this.#state.availableSchemas = schemas;
            this.#state.loaded = true;
        } catch (e) {
            this.#state.error = String(e);
        }
    }

    selectSchema(schemaId: string) {
        this.#state.selectedSchemaId = schemaId;
    }

    startValidation() {
        this.#state.isValidating = true;
        this.#state.error = null;
    }

    setResultKeepSchemas(result: ValidationResult) {
        this.#state.lastResult = result;
        this.#state.isValidating = false;
        this.#state.error = null;
    }
}

export const validationStore = new ValidationStore();
