export const convertAsync: (args: ConvertArgs) => Promise<number>

type ConvertArgs = {
  inputPath: string
  outputPath: string
}
