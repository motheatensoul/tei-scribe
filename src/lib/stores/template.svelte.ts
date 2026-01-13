import type { Template, AnnotationSchemaId } from '$lib/types/template';

export type { Template, AnnotationSchemaId };

class TemplateStore {
  #state = $state<{
    templates: Template[];
    active: Template | null;
  }>({
    templates: [],
    active: null,
  });

  get templates() { return this.#state.templates; }
  get active() { return this.#state.active; }

  setTemplates(templates: Template[]) {
    this.#state.templates = templates;
  }

  setActive(template: Template | null) {
    this.#state.active = template;
  }

  updateTemplate(template: Template) {
    this.#state.templates = this.#state.templates.map((t) =>
      t.id === template.id ? template : t
    );
    if (this.#state.active?.id === template.id) {
      this.#state.active = template;
    }
  }

  removeTemplate(id: string) {
    this.#state.templates = this.#state.templates.filter((t) => t.id !== id);
    if (this.#state.active?.id === id) {
      this.#state.active = null;
    }
  }

  reset() {
    this.#state.templates = [];
    this.#state.active = null;
  }
}

export const templateStore = new TemplateStore();
