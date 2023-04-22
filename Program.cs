using Discord;
using Discord.Interactions;
using Discord.WebSocket;

namespace POCBotButCSharp
{
	public class Program
	{
		private DiscordSocketClient? _client;
		private InteractionHandler? _interactionHandler;
		private readonly DiscordSocketConfig _socketConfig = new()
		{
			GatewayIntents = GatewayIntents.Guilds | GatewayIntents.GuildMembers,
			AlwaysDownloadUsers = true,
		};

		public static Task Main(string[] args) => new Program().MainAsync(args);
		public async Task MainAsync(string[] args)
		{
			// ...
			DotNetEnv.Env.Load();

			_client = new DiscordSocketClient(_socketConfig);
			_client.Log += Log;

			var _interactionService = new InteractionService(_client.Rest);
			_interactionHandler = new InteractionHandler(_client, _interactionService);
			await _interactionHandler.InitializeAsync();

			await _client.LoginAsync(TokenType.Bot, Environment.GetEnvironmentVariable("TOKEN"));
			await _client.StartAsync();
			await Task.Delay(-1);
		}

		private Task Log(LogMessage msg)
		{
			Console.WriteLine(msg.ToString());
			return Task.CompletedTask;
		}

		public static bool IsDebug()
		{
			bool debug = false;

			// Check if a command-line argument named "debug" is present
			string[] args = Environment.GetCommandLineArgs();
			if (args.Contains("debug"))
			{
				debug = true;
			}

			return debug;
		}
	}
}
