export function removeNullChars(str: string): string {
    return str.replace(/\u0000/g, "");
}
