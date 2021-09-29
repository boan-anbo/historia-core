import fs from 'fs';
const event_parse = require('.');

 const loadFile = async () =>
     fs.readFile('./src/tests/cultural-revolution-article.txt', 'utf8', (err, data) => {
         if (err) {
             console.error(err)
             return
         }
         // console.log(data)
         let start = process.hrtime();
         const result = event_parse.process(data)
         let stop = process.hrtime(start)
         console.log(`Time Taken to execute: ${(stop[0] * 1e9 + stop[1])/1e9} seconds`)
         // console.log(result)
         fs.writeFileSync('./src/tests/cultural-revolution-article.json', JSON.stringify(result,null, 4))
     })



loadFile().then(() => {
    console.log("Finished")
})