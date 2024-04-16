export default {
  logo: <h1 className="font-bold text-4xl md:text-4xl lg:text-5xl">Hollow</h1>,
  project: {
    link: "https://github.com/txpipe/hollow",
  },
  chat: {
    link: "https://discord.gg/Vc3x8N9nz2",
  },
  footer: {
    text: "Hollow - TxPipe",
  },
  nextThemes: {
    defaultTheme: "dark",
  },
  docsRepositoryBase: "https://github.com/txpipe/hollow/tree/main/docs",
  useNextSeoProps() {
    return {
      titleTemplate: "%s – Hollow",
      description: "Hollow is an SDK for building Headless Cardano dApps",
      canonical: "https://hollow.txpipe.io",
      siteName: "Hollow",
      openGraph: {
        url: "https://hollow.txpipe.io",
        title: "Hollow",
        description: "Hollow is an SDK for building Headless Cardano dApps",
        images: [
          {
            url: "https://hollow.txpipe.io/logo.png",
            width: 209,
            height: 209,
            alt: "Hollow",
            type: "image/png",
          },
        ],
      },
    };
  },
};
