const colours = [0xFF1B1C, 0x368F8B, 0x160F29, 0xBE7C4D, 0x92140C, 0x353238, 0xBE5A38, 0xC1B4AE]

export function getRandomColour() {
    const index = Math.floor(Math.random() * colours.length);
    return colours[index];
}