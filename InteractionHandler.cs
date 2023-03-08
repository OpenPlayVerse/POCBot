using System.Reflection;
using Discord;
using Discord.Interactions;
using Discord.WebSocket;

namespace POCBotButCSharp
{
  public class InteractionHandler
  {
    private readonly DiscordSocketClient _client;
    private readonly InteractionService _interactionService;

    public InteractionHandler(DiscordSocketClient client, InteractionService interactionService)
    {
      _client = client;
      _interactionService = interactionService;
    }

    public async Task InitializeAsync()
    {
      // Process when the client is ready, so we can register our commands.
      _client.Ready += ReadyAsync;
      _interactionService.Log += Log;

      // Add the public modules that inherit InteractionModuleBase<T> to the InteractionService
      await _interactionService.AddModulesAsync(Assembly.GetEntryAssembly(), null);

      // Process the InteractionCreated payloads to execute Interactions commands
      _client.InteractionCreated += HandleInteraction;
    }

    private Task Log(LogMessage msg)
    {
      Console.WriteLine(msg.ToString());
      return Task.CompletedTask;
    }

    private async Task ReadyAsync()
    {
      // Context & Slash commands can be automatically registered, but this process needs to happen after the client enters the READY state.
      // Since Global Commands take around 1 hour to register, we should use a test guild to instantly update and test our commands.
      if (Program.IsDebug())
        await _interactionService.RegisterCommandsToGuildAsync(561598333911826504, true);
      else
        await _interactionService.RegisterCommandsGloballyAsync(true);
    }

    private async Task HandleInteraction(SocketInteraction interaction)
    {
      try
      {
        // Create an execution context that matches the generic type parameter of your InteractionModuleBase<T> modules.
        var context = new SocketInteractionContext(_client, interaction);

        // Execute the incoming command.
        var result = await _interactionService.ExecuteCommandAsync(context, null);

        if (!result.IsSuccess)
          switch (result.Error)
          {
            case InteractionCommandError.UnmetPrecondition:
              // implement
              break;
            default:
              break;
          }
      }
      catch
      {
        // If Slash Command execution fails it is most likely that the original interaction acknowledgement will persist. It is a good idea to delete the original
        // response, or at least let the user know that something went wrong during the command execution.
        if (interaction.Type is InteractionType.ApplicationCommand)
          await interaction.GetOriginalResponseAsync().ContinueWith(async (msg) => await msg.Result.DeleteAsync());
      }
    }
  }
}