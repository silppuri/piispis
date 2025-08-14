// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!

import("./pkg")
  .then((m) => {
    console.log("Available exports:", Object.keys(m.default));
    console.log("main function:", typeof m.default);
    m.default.main();
  })
  .catch(console.error);
