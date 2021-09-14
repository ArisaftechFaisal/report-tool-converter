import * as fs from 'fs'
import * as util from 'util'

import test, { ExecutionContext } from 'ava'
import _ from 'lodash'

import { convertAsync } from '../index'

const readFile = util.promisify(fs.readFile, 'utf8')

const testPath = 'resources/'
const inputPaths = {
  dropdown: testPath + 'test_dropdown.xlsx',
  text: testPath + 'test_text.xlsx',
  textarea: testPath + 'test_textarea.xlsx',
  multiselect: testPath + 'test_multiselect.xlsx',
  radio: testPath + 'test_radio.xlsx',
  failNotExist: testPath + 'does_not_exist.xlsx',
  failWrongFormat: testPath + 'test_wrong_format.xlsx',
}

const expectedPaths = {
  dropdown: testPath + 'test_dropdown_expected.json',
  text: testPath + 'test_text_expected.json',
  textarea: testPath + 'test_textarea_expected.json',
  multiselect: testPath + 'test_multiselect_expected.json',
  radio: testPath + 'test_radio_expected.json',
}

const outputPaths = {
  dropdown: testPath + 'test_dropdown_output.json',
  text: testPath + 'test_text_output.json',
  textarea: testPath + 'test_textarea_output.json',
  multiselect: testPath + 'test_multiselect_output.json',
  radio: testPath + 'test_radio_output.json',
  failOutput: testPath + 'error.json',
}

test('convert test for dropdowns', async (t) => {
  await testConvert(t, inputPaths.dropdown, outputPaths.dropdown, expectedPaths.dropdown)
})

test('convert test for text', async (t) => {
  await testConvert(t, inputPaths.text, outputPaths.text, expectedPaths.text)
})

test('convert test for textarea', async (t) => {
  await testConvert(t, inputPaths.textarea, outputPaths.textarea, expectedPaths.textarea)
})

test('convert test for multiselect', async (t) => {
  await testConvert(t, inputPaths.multiselect, outputPaths.multiselect, expectedPaths.multiselect)
})

test('convert test for radio', async (t) => {
  await testConvert(t, inputPaths.radio, outputPaths.radio, expectedPaths.radio)
})

test('error on does not exist', async (t) => {
  try {
    await testConvert(t, inputPaths.failNotExist, outputPaths.failOutput, expectedPaths.radio)
  } catch (err: any) {
    t.assert(err.message)
  }
})

test('error on wrong format', async (t) => {
  try {
    await testConvert(t, inputPaths.failWrongFormat, outputPaths.failOutput, expectedPaths.radio)
  } catch (err: any) {
    t.assert(err.message)
  }
})

const testConvert = async (
  t: ExecutionContext<unknown>,
  inputPath: string,
  outputPath: string,
  expectedPath: string,
) => {
  await convertAsync({ inputPath, outputPath })
  const expected = await readJson(expectedPath)
  const output = await readJson(outputPath)
  const isEq = _.isEqual(output, expected)
  t.is(isEq, true)
}

const readJson = async (path: string): Promise<any> => {
  try {
    const cont = await readFile(path)
    return JSON.parse(cont)
  } catch (e) {
    console.error(e)
  }
}
