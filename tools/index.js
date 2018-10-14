const fs = require("fs-extra");
const globby = require("globby");
const os = require("os");
const path = require("path");
const pMap = require("p-map");
const ProgressBar = require("progress");
const puppeteer = require("puppeteer");

const { siteDir, outputDir } = require("minimist")(process.argv.slice(2));

async function takeAllScreenshots(siteDir, outputDir) {
  // we're only looking at our own pages here
  console.log("starting puppeteer without a sandbox");
  const browser = await puppeteer.launch({
    args: ["--no-sandbox", "--disable-setuid-sandbox"]
  });

  console.log(`taking screenshots of all pages in ${siteDir}`);
  console.log(`writing screenshots to ${outputDir}`);

  const pages = await globby(`${siteDir}/**/*.html`);

  const progressBar = new ProgressBar(
    "screenshotting :current of :total, estimated :eta s remaining",
    {
      total: pages.length
    }
  );

  await pMap(
    pages,
    async p => {
      let url = `file://${p}`;
      let screenshotPath = p
        .replace(siteDir, outputDir)
        .replace(".html", ".png");

      let isIndex = url.endsWith("index.html");

      await fs.mkdirp(path.dirname(screenshotPath));

      const page = await browser.newPage();

      await page.goto(url);

      await page.setViewport({ width: 1366, height: isIndex ? 3000 : 768 });

      await page.screenshot({
        path: screenshotPath,
        fullPage: !isIndex
      });

      await page.setViewport({
        width: 411,
        height: isIndex ? 3000 : 731,
        isMobile: true,
        hasTouch: true
      });

      await page.screenshot({
        path: screenshotPath.replace(".png", ".mobile.png"),
        fullPage: !isIndex
      });

      await page.close();
      progressBar.tick();
    },
    { concurrency: os.cpus().length }
  );

  console.log("closing puppeteer");
  await browser.close();
}

takeAllScreenshots(siteDir, outputDir)
  .then(() => {
    console.log("all done!");
  })
  .catch(e => {
    console.error(e);
    process.exit(1);
  });
