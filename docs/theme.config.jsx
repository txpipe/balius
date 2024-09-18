export default {
  logo: <h1 className="font-bold text-4xl md:text-4xl lg:text-5xl">Balius</h1>,
  project: {
    link: "https://github.com/txpipe/balius",
  },
  chat: {
    link: "https://discord.gg/Vc3x8N9nz2",
  },
  footer: {
    text: "Balius by TxPipe",
  },
  nextThemes: {
    defaultTheme: "dark",
  },
  docsRepositoryBase: "https://github.com/txpipe/balius/tree/main/docs",
  useNextSeoProps() {
    return {
      titleTemplate: "%s – Balius",
      description: "Balius is an SDK for building Headless dApps for UTxO-based blockchains",
      canonical: "https://balius.txpipe.io",
      siteName: "Balius",
      openGraph: {
        url: "https://balius.txpipe.io",
        title: "Balius",
        description: "Balius is an SDK for building Headless dApps for UTxO-based blockchains",
        images: [
          {
            url: "https://balius.txpipe.io/logo.png",
            width: 209,
            height: 209,
            alt: "Balius",
            type: "image/png",
          },
        ],
      },
    };
  },
};
