export interface AppError {
    id: string;
    timestamp: Date;
    level: 'error' | 'warning' | 'info';
    source: string;
    message: string;
    details?: string;
}

class ErrorStore {
    #errors = $state<AppError[]>([]);
    #idCounter = 0;

    get all() { return this.#errors; }
    
    // Derived counts
    get counts() {
        return {
            error: this.#errors.filter((e) => e.level === 'error').length,
            warning: this.#errors.filter((e) => e.level === 'warning').length,
            info: this.#errors.filter((e) => e.level === 'info').length,
            total: this.#errors.length,
        };
    }

    add(error: Omit<AppError, 'id' | 'timestamp'>) {
        const newError: AppError = {
            ...error,
            id: `error-${++this.#idCounter}`,
            timestamp: new Date(),
        };
        console.log(`[${error.level.toUpperCase()}] ${error.source}: ${error.message}`, error.details || '');
        
        // Use a temporary variable to update state properly in Svelte 5 if needed, 
        // though direct array assignment/push on $state works if it's the whole array
        const updated = [...this.#errors, newError].slice(-50);
        this.#errors = updated;
        return newError.id;
    }

    error(source: string, message: string, details?: string) {
        console.error(`[ERROR] ${source}: ${message}`, details || '');
        this.add({ level: 'error', source, message, details });
    }

    warning(source: string, message: string, details?: string) {
        console.warn(`[WARNING] ${source}: ${message}`, details || '');
        this.add({ level: 'warning', source, message, details });
    }

    info(source: string, message: string, details?: string) {
        console.info(`[INFO] ${source}: ${message}`, details || '');
        this.add({ level: 'info', source, message, details });
    }

    remove(id: string) {
        this.#errors = this.#errors.filter((e) => e.id !== id);
    }

    clear() {
        this.#errors = [];
    }
}

export const errorStore = new ErrorStore();
