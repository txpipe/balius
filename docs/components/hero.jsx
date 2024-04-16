export function Hero() {
  return (
    <div className="py-10 md:p-20">
      <div className="mb-5 text-center">
        <div className="flex justify-center">
          <img src="/logo.png" alt="Hollow logo" className="mb-4" />
        </div>
        <h1 className="flex flex-col flex-wrap font-bold text-4xl md:text-6xl lg:text-7xl dark:text-gray-200">
          SDK for building dApps
        </h1>
      </div>

      <div className="mb-8 max-w-3xl text-center mx-auto">
        <p className="md:text-lg text-gray-600 dark:text-gray-400">
          Hollow SDK streamlines the development of headless Cardano dApps by providing essential components out-of-the-box. Developers can focus on their business logic while leveraging the SDK's plumbing for on-chain interactions, promoting code reusability and scalability.
        </p>
      </div>

      <div className="flex justify-center">
        <a
          className="bg-gradient-to-tl from-emerald-600 to-cyan-600 border border-transparent text-white text-sm rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-offset-2 focus:ring-offset-white py-3 px-4 dark:focus:ring-offset-gray-800"
          href="/introduction"
        >
          Documentation
        </a>
      </div>
    </div>
  );
}
