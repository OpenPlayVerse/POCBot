using System.ComponentModel;
using Discord.Commands;
using Discord.Interactions;
using Discord.WebSocket;
using Newtonsoft.Json;

namespace POCBotButCSharp
{
  public class InfoModule : InteractionModuleBase<SocketInteractionContext>
  {
    [SlashCommand("ping", "Pong!")]
    public async Task PingAsync()
    {
      await RespondAsync("gnip!");
    }

    [SlashCommand("checkserver", "Checks if the Minecraft server is alive.")]
    public async Task CheckServerAsync([Choice("Project OpenComputers 3", "poc3.namelessserver.net"), Choice("Warpy", "warpy.namelessserver.net")] string server)
    {
      var client = new HttpClient();
      var response = await client.GetAsync($"https://api.mcsrvstat.us/2/{server}");
      var content = await response.Content.ReadAsStringAsync();
      var status = JsonConvert.DeserializeObject<QuickType.ServerData>(content);

      if (status != null && status.Players != null && status.Online)
      {
        var serverInfoEmbed = new Discord.EmbedBuilder()
          .WithTitle($"Server Info for {server}: {(status.Online ? "✅" : "❌")}")
          .AddField("Total Players: ", status.Players.Online > 0 ? $"{status.Players.Online}/{status.Players.Max}" : "None", true)
          .WithDescription($"```{string.Join(" ", status.Motd.Raw)}```")
          .WithColor(Discord.Color.Blue)
          .Build();
        await RespondAsync(embed: serverInfoEmbed);
      }
    }

    [Discord.Interactions.Group("news", "Lets you subscribe and unsubscribe to news")]
    public class NewsGroup : InteractionModuleBase<SocketInteractionContext>
    {
      public enum NewsRoles : ulong
      {
        [ChoiceDisplay("Server updates (News Letter)")]
        ServerUpdates = 805078371725869066,
        [ChoiceDisplay("Technical updates (Devlog Subscriber)")]
        TechnicalUpdates = 944371601560969326

      }

      [SlashCommand("subscribe", "Subscribe to news")]
      public async Task SubscribeAsync([Choice("Server updates (News Letter)", "Technical updates (Devlog Subscriber)")] NewsRoles role)
      {
        if (Context.User is not SocketGuildUser user || Context.Guild is not SocketGuild guild)
        {
          await ReplyAsync("You must be in a guild to use this command.");
          return;
        }

        var roleToAdd = guild.GetRole((ulong)role);
        await user.AddRoleAsync(roleToAdd);
        await RespondAsync($"Added role {roleToAdd.Name} to {user.Username}");
      }

      [SlashCommand("unsubscribe", "Unsubscribe from news")]
      public async Task UnsubscribeAsync([Choice("Server updates (News Letter)", "Technical updates (Devlog Subscriber)")] NewsRoles role)
      {
        if (Context.User is not SocketGuildUser user || Context.Guild is not SocketGuild guild)
        {
          await ReplyAsync("You must be in a guild to use this command.");
          return;
        }

        var roleToRemove = guild.GetRole((ulong)role);
        await user.RemoveRoleAsync(roleToRemove);
        await RespondAsync($"Removed role {roleToRemove.Name} from {user.Username}");
      }

    }
  }
}
