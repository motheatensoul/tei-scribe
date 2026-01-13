import type { EntityMap } from '$lib/types/entities';

/**
 * Resolves entity placeholders in text to their actual glyphs.
 * Used to restore entities after XML processing or for direct display.
 * 
 * @param text The text containing placeholders (e.g., __ENTITY_0__)
 * @param entityMap Map of placeholder string to entity name
 * @param entityDefs Registry of entity definitions containing glyphs
 * @returns Text with placeholders replaced by glyphs
 */
export function resolveEntitiesToGlyphs(
    text: string, 
    entityMap: Map<string, string>, 
    entityDefs: EntityMap
): string {
    let result = text;
    for (const [placeholder, entityName] of entityMap) {
        // Look up the entity in our entity definitions
        const entity = entityDefs[entityName];
        const glyph = entity?.char || `[${entityName}]`;
        result = result.replaceAll(placeholder, glyph);
    }
    return result;
}

/**
 * Replaces XML entity references (&name;) with unique placeholders.
 * Useful for processing XML with custom entities that aren't defined in the DTD/Schema.
 * 
 * @param xml The XML string to process
 * @returns Object containing the processed XML and the entity map for restoration
 */
export function replaceEntitiesWithPlaceholders(xml: string): { 
    processedXml: string; 
    entityMap: Map<string, string> 
} {
    const entityPattern = /&([a-zA-Z][a-zA-Z0-9]*);/g;
    const entityMap = new Map<string, string>();
    let entityCounter = 0;

    const processedXml = xml.replace(entityPattern, (match, name) => {
        // Skip standard XML entities
        if (['lt', 'gt', 'amp', 'quot', 'apos'].includes(name)) {
            return match;
        }
        const placeholder = `__ENTITY_${entityCounter}__`;
        entityMap.set(placeholder, name);
        entityCounter++;
        return placeholder;
    });

    return { processedXml, entityMap };
}
