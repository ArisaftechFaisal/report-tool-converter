export const sync: (input: number) => number
// sleep [duration] ms, return Promise which resolved 2 * duration
export const sleep: (duration: number) => Promise<number>
export const convertAsync: (args: ConvertArgs) => Promise<number>

type ConvertArgs = {
  inputPath: string
  outputPath: string
}
