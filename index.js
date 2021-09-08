const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'converter' means native addon name is `converter`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `converter.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `report-tool-converter-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'converter', 'report-tool-converter')
