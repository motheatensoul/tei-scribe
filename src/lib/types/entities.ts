export interface Entity {
    unicode: string;
    char: string;
    description: string;
    category: string;
}

export interface EntityMap {
    [name: string]: Entity;
}
