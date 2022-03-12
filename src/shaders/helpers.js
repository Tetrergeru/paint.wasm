export function uniformTexture(gl, location, texture) {
    gl.uniform1i(location, texture)
}

export function debugCanvases(c1, c2) {
    const c = [c1, c2]

    const thumbnails = [
        document.querySelector("#layer-canvas-1").getContext("2d"),
        document.querySelector("#layer-canvas-2").getContext("2d")
    ]

    thumbnails.forEach((it, idx) => {
        it.canvas.width = 100
        it.canvas.height = 100
        it.scale(it.canvas.width / c[idx].width, it.canvas.height / c[idx].height)
    })

    thumbnails[0].drawImage(c[0], 0, 0)
    thumbnails[1].drawImage(c[1], 0, 0)
}