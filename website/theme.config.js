import { useRouter } from "next/router";

const Logo = ({ height }) => (
  <svg height={height} viewBox="0 0 100 100" fill="none">
    <image xlinkHref="/img/logo.png" x="0" y="0" height="100" width="100" />
  </svg>
);

const TITLE_WITH_TRANSLATIONS = {
  "en-US": "Relational to Graph transformation tool. Fast."
};

const config = {
  github: "https://github.com/rtg-project/rtg",
  docsRepositoryBase: "https://github.com/rtg-project/rtg/blob/master/website/pages",
  titleSuffix: " – rtg",
  search: true,
  unstable_stork: true,
  floatTOC: true,
  logo: () => {
    const { locale } = useRouter();
    return (
      <>
        <Logo height={18} />
        <span className="mx-2 font-extrabold hidden md:inline select-none uppercase">
          rtg
        </span>
        <span className="text-gray-600 font-normal hidden lg:!inline whitespace-no-wrap">
          {TITLE_WITH_TRANSLATIONS[locale]}
        </span>
      </>
    );
  },
  head: ({ title, meta }) => {
    return (
      <>
        {/* Favicons, meta */}
        <link
          rel="apple-touch-icon"
          sizes="180x180"
          href="/favicon/apple-touch-icon.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="32x32"
          href="/favicon/favicon-32x32.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="16x16"
          href="/favicon/favicon-16x16.png"
        />
        <link rel="manifest" href="/favicon/site.webmanifest" />
        <link
          rel="mask-icon"
          href="/favicon/safari-pinned-tab.svg"
          color="#000000"
        />
        <meta name="msapplication-TileColor" content="#ffffff" />
        <meta name="theme-color" content="#ffffff" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta httpEquiv="Content-Language" content="en" />
        <meta
          name="description"
          content={
            meta.description ||
            "rtg is a Relational to Graph transformation tool. Fast."
          }
        />
        <meta
          name="og:description"
          content={
            meta.description ||
            "rtg is a Relational to Graph transformation tool. Fast."
          }
        />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:site" content="@vercel" />
        <meta
          name="twitter:image"
          content={
            meta.image ||
            "https://assets.vercel.com/image/upload/v1572282926/rtg/twitter-card.jpg"
          }
        />
        <meta
          name="og:title"
          content={
            title ? title + " – rtg" : "rtg: Relational to Graph transformation tool. Fast."
          }
        />
        <meta
          name="og:image"
          content={
            meta.image ||
            "https://assets.vercel.com/image/upload/v1572282926/rtg/twitter-card.jpg"
          }
        />
        <meta name="apple-mobile-web-app-title" content="rtg" />
        <link
          rel="stylesheet"
          href="https://cdn.jsdelivr.net/npm/docsearch.js@2/dist/cdn/docsearch.min.css"
          media="print"
          onLoad="this.media='all'"
        />
      </>
    );
  },
  footerEditLink: ({ locale }) => {
    switch (locale) {
      default:
        return "Edit this page on GitHub";
    }
  },
  footerText: ({ locale }) => {
    switch (locale) {
      default:
        return (
          null
        );
    }
  },
  i18n: [
    { locale: "en-US", text: "English" },
    // { locale: "es-ES", text: "Español" },
    // { locale: "zh-CN", text: "简体中文" },
    // { locale: "ja", text: "日本語" },
    // { locale: "ko", text: "한국어" },
  ],
};

export default config;