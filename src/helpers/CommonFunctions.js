export function getWidth() {
    return Math.max(
        document.body.scrollWidth,
        document.documentElement.scrollWidth,
        document.body.offsetWidth,
        document.documentElement.offsetWidth,
        document.documentElement.clientWidth
    );
}


export function removeFromArray(arr, toRemove) {
    arr = arr.filter(element => element !== toRemove);
}