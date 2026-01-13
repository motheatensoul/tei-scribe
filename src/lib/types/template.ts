/**
 * Annotation schema identifiers for template-coupled annotations.
 * Each schema defines which annotation types and options are available.
 */
export type AnnotationSchemaId = "tei-p5" | "menota";

export interface Template {
  id: string;
  name: string;
  description: string;
  header: string;
  footer: string;
  wordWrap: boolean;
  autoLineNumbers: boolean;
  multiLevel: boolean;
  wrapPages: boolean;
  validationSchemaId?: string;
  /**
   * Which annotation schema to use for this template.
   * - "tei-p5": Generic TEI P5 annotations
   * - "menota": MENOTA-specific annotations with proper attributes
   * Defaults to "menota" for multiLevel templates, "tei-p5" otherwise.
   */
  annotationSchemaId?: AnnotationSchemaId;
}
