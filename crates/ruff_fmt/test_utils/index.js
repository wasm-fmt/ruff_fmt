/**
 * @typedef {Object} SnapshotInfo
 * @property {string} input_file
 * @property {string} extension
 */

/**
 * Parse binary snapshot metadata file.
 * The actual content is in a separate file with the extension specified in metadata.
 * @param {string} content - The content of the .snap metadata file
 * @returns {SnapshotInfo | null}
 */
export function parseSnapshot(content) {
    const match = content.match(/^---\n([\s\S]*?)\n---\n?$/);
    if (!match) return null;

    const header = match[1];
    const inputFileMatch = header.match(/^input_file:\s*(.+)$/m);
    if (!inputFileMatch) return null;

    const extensionMatch = header.match(/^extension:\s*(.+)$/m);
    if (!extensionMatch) return null;

    return {
        input_file: inputFileMatch[1],
        extension: extensionMatch[1],
    };
}
