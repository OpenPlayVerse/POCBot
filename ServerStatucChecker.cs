namespace QuickType
{
	using System.Collections.Generic;

	using System.Globalization;
	using Newtonsoft.Json;
	using Newtonsoft.Json.Converters;

	public partial class ServerData
	{
		[JsonProperty("ip")]
		public string? Ip { get; set; }

		[JsonProperty("port")]
		public long Port { get; set; }

		[JsonProperty("debug")]
		public Debug? Debug { get; set; }

		[JsonProperty("motd")]
		public Motd? Motd { get; set; }

		[JsonProperty("players")]
		public Players? Players { get; set; }

		[JsonProperty("version")]
		public string? Version { get; set; }

		[JsonProperty("online")]
		public bool Online { get; set; }

		[JsonProperty("protocol")]
		public long Protocol { get; set; }

		[JsonProperty("hostname")]
		public string? Hostname { get; set; }

		[JsonProperty("icon")]
		public string? Icon { get; set; }

		[JsonProperty("mods")]
		public Mods? Mods { get; set; }
	}

	public partial class Debug
	{
		[JsonProperty("ping")]
		public bool Ping { get; set; }

		[JsonProperty("query")]
		public bool Query { get; set; }

		[JsonProperty("srv")]
		public bool Srv { get; set; }

		[JsonProperty("querymismatch")]
		public bool Querymismatch { get; set; }

		[JsonProperty("ipinsrv")]
		public bool Ipinsrv { get; set; }

		[JsonProperty("cnameinsrv")]
		public bool Cnameinsrv { get; set; }

		[JsonProperty("animatedmotd")]
		public bool Animatedmotd { get; set; }

		[JsonProperty("cachetime")]
		public long Cachetime { get; set; }

		[JsonProperty("cacheexpire")]
		public long Cacheexpire { get; set; }

		[JsonProperty("apiversion")]
		public long Apiversion { get; set; }

		[JsonProperty("dns")]
		public Dns? Dns { get; set; }

		[JsonProperty("error")]
		public Error? Error { get; set; }
	}

	public partial class Dns
	{
		[JsonProperty("srv_a")]
		public SrvA[]? SrvA { get; set; }

		[JsonProperty("srv")]
		public Srv[]? Srv { get; set; }
	}

	public partial class Srv
	{
		[JsonProperty("name")]
		public string? Name { get; set; }

		[JsonProperty("type")]
		public string? Type { get; set; }

		[JsonProperty("class")]
		public string? Class { get; set; }

		[JsonProperty("ttl")]
		public long Ttl { get; set; }

		[JsonProperty("rdlength")]
		public long Rdlength { get; set; }

		[JsonProperty("rdata")]
		public string? Rdata { get; set; }

		[JsonProperty("priority")]
		public long Priority { get; set; }

		[JsonProperty("weight")]
		public long Weight { get; set; }

		[JsonProperty("port")]
		public long Port { get; set; }

		[JsonProperty("target")]
		public string? Target { get; set; }
	}

	public partial class SrvA
	{
		[JsonProperty("name")]
		public string? Name { get; set; }

		[JsonProperty("type")]
		public string? Type { get; set; }

		[JsonProperty("class")]
		public string? Class { get; set; }

		[JsonProperty("ttl")]
		public long Ttl { get; set; }

		[JsonProperty("rdlength")]
		public long Rdlength { get; set; }

		[JsonProperty("rdata")]
		public string? Rdata { get; set; }

		[JsonProperty("cname", NullValueHandling = NullValueHandling.Ignore)]
		public string? Cname { get; set; }

		[JsonProperty("address", NullValueHandling = NullValueHandling.Ignore)]
		public string? Address { get; set; }
	}

	public partial class Error
	{
		[JsonProperty("query")]
		public string? Query { get; set; }
	}

	public partial class Mods
	{
		[JsonProperty("names")]
		public string[]? Names { get; set; }

		[JsonProperty("raw")]
		public Dictionary<string, string>? Raw { get; set; }
	}

	public partial class Motd
	{
		[JsonProperty("raw")]
		public string[]? Raw { get; set; }

		[JsonProperty("clean")]
		public string[]? Clean { get; set; }

		[JsonProperty("html")]
		public string[]? Html { get; set; }
	}

	public partial class Players
	{
		[JsonProperty("online")]
		public long Online { get; set; }

		[JsonProperty("max")]
		public long Max { get; set; }
	}

	public partial class ServerData
	{
		public static ServerData? FromJson(string json) => JsonConvert.DeserializeObject<ServerData>(json, QuickType.Converter.Settings);
	}

	public static class Serialize
	{
		public static string ToJson(this ServerData self) => JsonConvert.SerializeObject(self, QuickType.Converter.Settings);
	}

	internal static class Converter
	{
		public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
		{
			MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
			DateParseHandling = DateParseHandling.None,
			Converters = {
		new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
	  },
		};
	}
}
