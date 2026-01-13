import { replaceEntitiesWithPlaceholders, resolveEntitiesToGlyphs } from './entities';
import type { EntityMap } from '$lib/types/entities';

/**
 * Applies an XSLT transformation to XML content.
 * Handles entity preservation by using placeholders during transformation.
 * 
 * @param xmlContent The raw XML content
 * @param xslText The XSLT stylesheet content
 * @param entities Entity definitions for resolving placeholders after transformation
 * @returns Transformed HTML string
 */
export async function applyXsltTransform(
    xmlContent: string,
    xslText: string,
    entities: EntityMap
): Promise<string> {
    const parser = new DOMParser();
    
    // Parse XSL
    const xslDoc = parser.parseFromString(xslText, 'application/xml');
    const xslParseError = xslDoc.querySelector('parsererror');
    if (xslParseError) {
        throw new Error(`XSL parse error: ${xslParseError.textContent}`);
    }

    const processor = new XSLTProcessor();
    processor.importStylesheet(xslDoc);

    // Prepare XML by replacing entities with placeholders
    const { processedXml, entityMap } = replaceEntitiesWithPlaceholders(xmlContent);

    // Parse XML
    const xmlDoc = parser.parseFromString(processedXml, 'application/xml');
    const xmlParseError = xmlDoc.querySelector('parsererror');
    if (xmlParseError) {
        throw new Error(`XML parse error: ${xmlParseError.textContent}`);
    }

    // Transform
    const resultDoc = processor.transformToDocument(xmlDoc);
    if (!resultDoc || !resultDoc.documentElement) {
        throw new Error('XSLT transformation produced no output');
    }

    // Get HTML content
    let html = resultDoc.documentElement.outerHTML;

    // Resolve entities back to glyphs
    return resolveEntitiesToGlyphs(html, entityMap, entities);
}
