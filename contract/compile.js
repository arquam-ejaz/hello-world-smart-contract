
const sh = require('shelljs')



const calledFromDir = sh.pwd().toString()



sh.cd(__dirname)



const debug = process.argv.pop() === '--debug'





const buildCmd = debug
  ? 'cargo build --target wasm32-unknown-unknown'
  : 'cargo build --target wasm32-unknown-unknown --release'


const { code } = sh.exec(buildCmd)





if (code === 0 && calledFromDir !== __dirname) {
  const linkDir = `${calledFromDir}/out`
  const link = `${calledFromDir}/out/main.wasm`
  const packageName = require('fs').readFileSync(`${__dirname}/Cargo.toml`).toString().match(/name = "([^"]+)"/)[1]
  const outFile = `./target/wasm32-unknown-unknown/${debug ? 'debug' : 'release'}/${packageName}.wasm`
  sh.mkdir('-p', linkDir)
  sh.rm('-f', link)

  sh.cp('-u', outFile, link)
}


process.exit(code)
