export type YTAccessibility = {
  accessibilityData: {
    label: string
  }
}

export type YTRuns = {
  text: string
}[]

export type YTVideo = {
  videoRenderer: {
    videoId: string
    thumbnail: {
      thumbnails: {
        url: string
        width: number
        height: number
      }[]
    }
    title: {
      runs: YTRuns
      accessibility: YTAccessibility
    }
    descriptionSnippet: {
      runs: YTRuns
    } | null
    longBylineText: any
    publishedTimeText: {
      simpleText: string
    }
    lengthText: {
      accessibility: YTAccessibility
      simpleText: string
    }
    viewCountText: {
      simpleText: string
    }
    navigationEndpoint: {
      clickTrackingParams: any
      commandMetadata: {
        webCommandMetadata: {
          url: string
          webPageType: string
          rootVe: string
        }
      }
    }
  }
}

export type YTSectionRenderer = { contents: any[] }

export type YTResponse = {
  responseContext: object
  estimatedResults: number
  contents: {
    twoColumnSearchResultsRenderer: {
      primaryContents: {
        sectionListRenderer: {
          contents: YTSectionRenderer | any
        }
      }
    }
  }
}

export type Video = {
  thumbnail: string
  title: string
  desc: string
  author: string
  authorChannel: string
  published: string
  length: string
  views: string
  url: string
}
