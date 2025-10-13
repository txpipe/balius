// Run with:
// $ deno run --allow-net api_server.ts

Deno.serve({ port: 8080 }, async (req) => {
  const start = Date.now();
  const url = new URL(req.url);

  let response: Response;

  switch (url.pathname) {
      case "/ship":
        const name = url.searchParams.get("name");
        const pos_x = url.searchParams.get("x");
        const pos_y = url.searchParams.get("y");
        const fuel = url.searchParams.get("fuel");
        console.log(`Detected ship ${name} in position (${pos_x},${pos_y}) with fuel ${fuel}`);
        response = new Response(
          JSON.stringify({ greeting: `Hello, ${name}!` }),
          { headers: { "Content-Type": "application/json" } }
        );
        break;
  
    default:
      response = new Response(
        JSON.stringify({ error: "Not Found" }),
        { status: 404, headers: { "Content-Type": "application/json" } }
      );
  }

  // Log request after generating the response
  const ms = Date.now() - start;
  console.log(
    `${req.method} ${url} → ${response.status} (${ms}ms)`
  );

  return response;
});
