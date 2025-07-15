export interface Manifest {
    latest: {
        release: string,
        snapshot: string
    },
    versions: {
        id: string,
        type: string,
        url: string,
        time: Date,
        releaseTime: Date
    }[]
}