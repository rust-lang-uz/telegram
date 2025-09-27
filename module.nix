# Template & Guide from
# https://github.com/reckenrode/nix-foundryvtt/blob/main/modules/foundryvtt/default.nix
flake: {
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.rustina;
  bot = flake.packages.${pkgs.stdenv.hostPlatform.system}.default;

  genArgs = {cfg}: let
    telegram = cfg.tokens.telegram;
    github = cfg.tokens.github;
    domain = cfg.webhook.domain or "";
    mode =
      if cfg.webhook.enable
      then "webhook"
      else "polling";
    port =
      if cfg.webhook.enable
      then "--port ${toString cfg.webhook.port}"
      else "";
  in
    lib.strings.concatStringsSep " " [mode telegram github domain port];

  caddy = lib.mkIf (cfg.enable && cfg.webhook.enable && cfg.webhook.proxy == "caddy") {
    services.caddy.virtualHosts = lib.debug.traceIf (builtins.isNull cfg.webhook.domain) "webhook.domain can't be null, please specicy it properly!" {
      "${cfg.webhook.domain}" = {
        extraConfig = ''
          reverse_proxy 127.0.0.1:${toString cfg.webhook.port}
        '';
      };
    };
  };

  nginx = lib.mkIf (cfg.enable && cfg.webhook.enable && cfg.webhook.proxy == "nginx") {
    services.nginx.virtualHosts = lib.debug.traceIf (builtins.isNull cfg.webhook.domain) "webhook.domain can't be null, please specicy it properly!" {
      "${cfg.webhook.domain}" = {
        addSSL = true;
        enableACME = true;
        locations."/" = {
          proxyPass = "http://127.0.0.1:${toString cfg.webhook.port}";
          proxyWebsockets = true;
        };
      };
    };
  };

  service = lib.mkIf cfg.enable {
    users.users.${cfg.user} = {
      description = "Rustina Bot management user";
      isSystemUser = true;
      group = cfg.group;
    };

    users.groups.${cfg.group} = {};

    systemd.services.rustina-bot = {
      description = "Rustina Bot for managing telegram community";
      documentation = ["https://rust-lang.uz/"];

      after = ["network-online.target"];
      wants = ["network-online.target"];
      wantedBy = ["multi-user.target"];

      serviceConfig = {
        User = cfg.user;
        Group = cfg.group;
        Restart = "always";
        ExecStart = "${lib.getBin cfg.package}/bin/bot ${genArgs {cfg = cfg;}}";
        StateDirectory = cfg.user;
        StateDirectoryMode = "0750";
        # EnvironmentFile = cfg.secret;
        CapabilityBoundingSet = [
          "AF_NETLINK"
          "AF_INET"
          "AF_INET6"
        ];
        DeviceAllow = ["/dev/stdin r"];
        DevicePolicy = "strict";
        IPAddressAllow = "localhost";
        LockPersonality = true;
        # MemoryDenyWriteExecute = true;
        NoNewPrivileges = true;
        PrivateDevices = true;
        PrivateTmp = true;
        PrivateUsers = true;
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectSystem = "strict";
        ReadOnlyPaths = ["/"];
        RemoveIPC = true;
        RestrictAddressFamilies = [
          "AF_NETLINK"
          "AF_INET"
          "AF_INET6"
        ];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = "native";
        SystemCallFilter = [
          "@system-service"
          "~@privileged"
          "~@resources"
          "@pkey"
        ];
        UMask = "0027";
      };

      # preStart = ''
      #   installedConfigFile="${config.services.rustina.dataDir}/Config/options.json"
      #   install -d -m750 ${config.services.rustina.dataDir}/Config
      #   rm -f "$installedConfigFile" && install -m640 ${configFile} "$installedConfigFile"
      # '';
    };
  };

  asserts = lib.mkIf cfg.enable {
    warnings = [
      (lib.mkIf (cfg.webhook.enable && cfg.webhook.domain == null) "services.rustina.webhook.domain must be set in order to properly generate certificate!")
    ];

    assertions = [
      {
        assertion = cfg.tokens.telegram != null;
        message = "services.rustina.tokens.telegram must be set!";
      }
      {
        assertion = cfg.tokens.github != null;
        message = "services.rustina.tokens.github must be set!";
      }
    ];
  };
in {
  options = with lib; {
    services.rustina = {
      enable = mkEnableOption ''
        Rustina Bot: Telegram bot made by Uzbek Rust team for Rust Uzbekistan community.
      '';

      webhook = {
        enable = mkEnableOption ''
          Webhook method of deployment
        '';

        domain = mkOption {
          type = with types; nullOr str;
          default = null;
          example = "rust-lang.uz";
          description = "Domain to use while adding configurations to web proxy server";
        };

        proxy = mkOption {
          type = with types;
            nullOr (enum [
              "nginx"
              "caddy"
            ]);
          default = "caddy";
          description = "Proxy reverse software for hosting webhook";
        };

        port = mkOption {
          type = types.int;
          default = 8451;
          description = "Port to use for passing over proxy";
        };
      };

      tokens = {
        telegram = mkOption {
          type = with types; nullOr path;
          default = null;
          description = ''
            Path to telegram bot token of Rustina manager.
          '';
        };

        github = mkOption {
          type = with types; nullOr path;
          default = null;
          description = ''
            Path to github token for Rustina manager.
          '';
        };
      };

      user = mkOption {
        type = types.str;
        default = "rustina-bot";
        description = "User for running system + accessing keys";
      };

      group = mkOption {
        type = types.str;
        default = "rustina-bot";
        description = "Group for running system + accessing keys";
      };

      dataDir = mkOption {
        type = types.str;
        default = "/var/lib/rustina/bot";
        description = ''
          The path where Rustina Bot keeps its config, data, and logs.
        '';
      };

      package = mkOption {
        type = types.package;
        default = bot;
        description = ''
          The Rustian Bot package to use with the service.
        '';
      };
    };
  };

  config = lib.mkMerge [asserts service caddy nginx];
}
