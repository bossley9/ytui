#!/usr/bin/env -S deno run --allow-net

// @ts-ignore
import { YTResponse, Video } from "./types.ts";
// @ts-ignore
import { BASE_URL, FQDN } from "./constants.ts";

/*
 * functions
 */

const parseArgs = (): string[] => {
  // @ts-ignore
  // apparently Deno is undefined
  return Deno.args;
};

const getSearchStr = (): string => {
  const searchArgs: string[] = parseArgs();
  return searchArgs.join(" ");
};

const urlEncode = (url: string): string => {
  return encodeURIComponent(url);
};

/*
 * main
 */

// pseudo main method bc I'm used to C at this point
const main = async (): Promise<void> => {
  const searchStr = getSearchStr();
  const encodedUrl = urlEncode(searchStr);
  const url = BASE_URL + encodedUrl;

  let raw: string = "";

  try {
    const res = await fetch(url, { method: "GET" });
    raw = await res.text();
  } catch (e) {
    console.error(e);
  }

  // condense to single line
  raw = raw.replace(/\n/g, "");
  // remove start
  raw = raw.replace(/^.*var\ ytInitialData\ =/, "");
  // remove end
  raw = raw.replace(/;<\/script>.*$/, "");

  let res: YTResponse = JSON.parse(raw);

  // console.log(JSON.stringify(res));

  let data: any =
    res.contents.twoColumnSearchResultsRenderer.primaryContents
      .sectionListRenderer.contents[0].itemSectionRenderer.contents;

  let videos: any = data
    .map((videoRaw: any) => {
      const {
        // horizontalCardListRenderer: h,
        // radioRenderer: r,
        // shelfRenderer: s,
        videoRenderer: v,
      } = videoRaw;

      let video: Video | null = null;

      if (v) {
        video = {
          thumbnail: v.thumbnail.thumbnails[0].url,
          title: v.title.runs[0].text,
          desc: v.descriptionSnippet
            ? v.descriptionSnippet.runs.map((r: any) => r.text).join(" ")
            : "",
          author: v.ownerText.runs[0].text,
          authorChannel:
            FQDN +
            v.ownerText.runs[0].navigationEndpoint.commandMetadata
              .webCommandMetadata.url,
          published: v.publishedTimeText ? v.publishedTimeText.simpleText : "",
          length: v.lengthText.simpleText,
          views: v.viewCountText.simpleText,
          url: `${FQDN}/watch?v=${v.videoId}`,
        };
      } else {
        video = null;
      }

      return video;
    })
    // remove undefined renderers
    .filter((v: Video | null) => v);

  console.log(
    videos
      .map((v: Video) => {
        const { author, length, title, url } = v;
        return `(${length}) ${author} - ${title} - ${url}`;
      })
      .join("\n")
  );
};

main();
