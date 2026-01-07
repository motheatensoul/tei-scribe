import { invoke } from '@tauri-apps/api/core';
import type { Template } from './stores/template';
import type { Entity, EntityMap } from './stores/entities';

export interface Settings {
    fontSize: number;
    theme: string;
    autoPreview: boolean;
    previewDelay: number;
    activeTemplateId: string | null;
}

export async function loadSettings(): Promise<Settings> {
    return invoke('load_settings');
}

export async function saveSettings(settings: Settings): Promise<void> {
    return invoke('save_settings', { settings });
}

export interface FileContent {
    path: string;
    content: string;
}

export async function openFile(path: string): Promise<FileContent> {
    return invoke('open_file', { path });
}

export async function saveFile(path: string, content: string): Promise<void> {
    return invoke('save_file', { path, content });
}

export async function exportTei(path: string, teiContent: string): Promise<void> {
    return invoke('export_tei', { path, teiContent });
}

export async function loadTextFile(path: string): Promise<string> {
    return invoke('load_text_file', { path });
}

export async function listTemplates(): Promise<Template[]> {
    return invoke('list_templates');
}

export async function getTemplate(id: string): Promise<Template> {
    return invoke('get_template', { id });
}

export async function saveTemplate(template: Template): Promise<void> {
    return invoke('save_template', { template });
}

export interface CompileOptions {
    wordWrap?: boolean;
    autoLineNumbers?: boolean;
    multiLevel?: boolean;
    entitiesJson?: string;
    normalizerJson?: string;
    entityMappingsJson?: string;
    customMappings?: Record<string, string>;
}

export async function compileDsl(
    input: string,
    templateHeader: string,
    templateFooter: string,
    options?: CompileOptions
): Promise<string> {
    return invoke('compile_dsl', {
        input,
        templateHeader,
        templateFooter,
        wordWrap: options?.wordWrap ?? false,
        autoLineNumbers: options?.autoLineNumbers ?? false,
        multiLevel: options?.multiLevel ?? false,
        entitiesJson: options?.entitiesJson ?? null,
        normalizerJson: options?.normalizerJson ?? null,
        entityMappingsJson: options?.entityMappingsJson ?? null,
        customMappings: options?.customMappings ?? null,
    });
}

// Entity functions
export async function loadEntities(path: string): Promise<EntityMap> {
    return invoke('load_entities', { path });
}

export async function getEntity(path: string, name: string): Promise<Entity | null> {
    return invoke('get_entity', { path, name });
}

export async function listEntityNames(path: string): Promise<string[]> {
    return invoke('list_entity_names', { path });
}

// Custom entity mapping functions
export async function loadCustomMappings(): Promise<Record<string, string>> {
    return invoke('load_custom_mappings');
}

export async function saveEntityMapping(entity: string, translation: string): Promise<void> {
    return invoke('save_entity_mapping', { entity, translation });
}

export async function removeEntityMapping(entity: string): Promise<void> {
    return invoke('remove_entity_mapping', { entity });
}

export async function clearCustomMappings(): Promise<void> {
    return invoke('clear_custom_mappings');
}

// ONP Dictionary types and functions

export interface OnpEntry {
    release: string;
    lemma: string;
    lemma_mod: string;
    language: string;
    id: string;
    part_of_speech: string[];
    formats: string[];
    citations: number;
}

export interface OnpSense {
    definition: string;
    key: string;
    def: string;
}

export interface OnpFullEntry {
    context?: string;
    entry_type?: string;
    language: string;
    part_of_speech: string;
    canonical_form: { written_rep: string };
    citations: string;
    canonical_url: string;
    senses: OnpSense[];
}

export interface InflectedForm {
    onp_id: string;
    lemma: string;
    analysis: string;
    part_of_speech: string;
}

export interface InflectionStore {
    forms: Record<string, InflectedForm[]>;
}

// Load ONP headwords from file
export async function loadOnpHeadwords(path: string): Promise<number> {
    return invoke('load_onp_headwords', { path });
}

// Look up lemma candidates for a wordform
export async function lookupLemma(wordform: string): Promise<OnpEntry[]> {
    return invoke('lookup_lemma', { wordform });
}

// Search for lemmas by prefix (autocomplete)
export async function searchLemmaPrefix(prefix: string, limit: number = 20): Promise<OnpEntry[]> {
    return invoke('search_lemma_prefix', { prefix, limit });
}

// Get a specific ONP entry by ID
export async function getOnpEntry(id: string): Promise<OnpEntry | null> {
    return invoke('get_onp_entry', { id });
}

// Fetch full entry data from ONP API
export async function fetchOnpFullEntry(id: string): Promise<OnpFullEntry> {
    return invoke('fetch_onp_full_entry', { id });
}

// Load user inflection mappings
export async function loadInflections(): Promise<InflectionStore> {
    return invoke('load_inflections');
}

// Look up inflections for a wordform
export async function lookupInflection(wordform: string): Promise<InflectedForm[]> {
    return invoke('lookup_inflection', { wordform });
}

// Add an inflection mapping
export async function addInflection(
    wordform: string,
    onpId: string,
    lemma: string,
    analysis: string,
    partOfSpeech: string
): Promise<void> {
    return invoke('add_inflection', {
        wordform,
        onp_id: onpId,
        lemma,
        analysis,
        part_of_speech: partOfSpeech
    });
}

// Remove an inflection mapping
export async function removeInflection(
    wordform: string,
    onpId: string,
    analysis: string
): Promise<void> {
    return invoke('remove_inflection', {
        wordform,
        onp_id: onpId,
        analysis
    });
}

// Clear all inflection mappings
export async function clearInflections(): Promise<void> {
    return invoke('clear_inflections');
}

// Check if ONP registry is loaded
export async function isOnpLoaded(): Promise<boolean> {
    return invoke('is_onp_loaded');
}

// Get ONP registry stats
export async function getOnpStats(): Promise<[number, number]> {
    return invoke('get_onp_stats');
}
