// package: twitchroulette.v1
// file: twitchroulette/v1/twitchroulette.proto

var twitchroulette_v1_twitchroulette_pb = require("../../twitchroulette/v1/twitchroulette_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var TwitchRouletteService = (function () {
  function TwitchRouletteService() {}
  TwitchRouletteService.serviceName = "twitchroulette.v1.TwitchRouletteService";
  return TwitchRouletteService;
}());

TwitchRouletteService.GetRandomStream = {
  methodName: "GetRandomStream",
  service: TwitchRouletteService,
  requestStream: false,
  responseStream: false,
  requestType: twitchroulette_v1_twitchroulette_pb.GetRandomStreamRequest,
  responseType: twitchroulette_v1_twitchroulette_pb.GetRandomStreamResponse
};

TwitchRouletteService.GetStreamTags = {
  methodName: "GetStreamTags",
  service: TwitchRouletteService,
  requestStream: false,
  responseStream: false,
  requestType: twitchroulette_v1_twitchroulette_pb.GetStreamTagsRequest,
  responseType: twitchroulette_v1_twitchroulette_pb.GetStreamTagsResponse
};

TwitchRouletteService.GetStreamGames = {
  methodName: "GetStreamGames",
  service: TwitchRouletteService,
  requestStream: false,
  responseStream: false,
  requestType: twitchroulette_v1_twitchroulette_pb.GetStreamGamesRequest,
  responseType: twitchroulette_v1_twitchroulette_pb.GetStreamGamesResponse
};

exports.TwitchRouletteService = TwitchRouletteService;

function TwitchRouletteServiceClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

TwitchRouletteServiceClient.prototype.getRandomStream = function getRandomStream(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(TwitchRouletteService.GetRandomStream, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

TwitchRouletteServiceClient.prototype.getStreamTags = function getStreamTags(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(TwitchRouletteService.GetStreamTags, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

TwitchRouletteServiceClient.prototype.getStreamGames = function getStreamGames(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(TwitchRouletteService.GetStreamGames, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

exports.TwitchRouletteServiceClient = TwitchRouletteServiceClient;

