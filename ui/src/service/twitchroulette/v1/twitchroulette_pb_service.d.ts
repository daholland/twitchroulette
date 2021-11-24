// package: twitchroulette.v1
// file: twitchroulette/v1/twitchroulette.proto

import * as twitchroulette_v1_twitchroulette_pb from "../../twitchroulette/v1/twitchroulette_pb";
import {grpc} from "@improbable-eng/grpc-web";

type TwitchRouletteServiceGetRandomStream = {
  readonly methodName: string;
  readonly service: typeof TwitchRouletteService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof twitchroulette_v1_twitchroulette_pb.GetRandomStreamRequest;
  readonly responseType: typeof twitchroulette_v1_twitchroulette_pb.GetRandomStreamResponse;
};

type TwitchRouletteServiceGetStreamTags = {
  readonly methodName: string;
  readonly service: typeof TwitchRouletteService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof twitchroulette_v1_twitchroulette_pb.GetStreamTagsRequest;
  readonly responseType: typeof twitchroulette_v1_twitchroulette_pb.GetStreamTagsResponse;
};

type TwitchRouletteServiceGetStreamGames = {
  readonly methodName: string;
  readonly service: typeof TwitchRouletteService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof twitchroulette_v1_twitchroulette_pb.GetStreamGamesRequest;
  readonly responseType: typeof twitchroulette_v1_twitchroulette_pb.GetStreamGamesResponse;
};

export class TwitchRouletteService {
  static readonly serviceName: string;
  static readonly GetRandomStream: TwitchRouletteServiceGetRandomStream;
  static readonly GetStreamTags: TwitchRouletteServiceGetStreamTags;
  static readonly GetStreamGames: TwitchRouletteServiceGetStreamGames;
}

export type ServiceError = { message: string, code: number; metadata: grpc.Metadata }
export type Status = { details: string, code: number; metadata: grpc.Metadata }

interface UnaryResponse {
  cancel(): void;
}
interface ResponseStream<T> {
  cancel(): void;
  on(type: 'data', handler: (message: T) => void): ResponseStream<T>;
  on(type: 'end', handler: (status?: Status) => void): ResponseStream<T>;
  on(type: 'status', handler: (status: Status) => void): ResponseStream<T>;
}
interface RequestStream<T> {
  write(message: T): RequestStream<T>;
  end(): void;
  cancel(): void;
  on(type: 'end', handler: (status?: Status) => void): RequestStream<T>;
  on(type: 'status', handler: (status: Status) => void): RequestStream<T>;
}
interface BidirectionalStream<ReqT, ResT> {
  write(message: ReqT): BidirectionalStream<ReqT, ResT>;
  end(): void;
  cancel(): void;
  on(type: 'data', handler: (message: ResT) => void): BidirectionalStream<ReqT, ResT>;
  on(type: 'end', handler: (status?: Status) => void): BidirectionalStream<ReqT, ResT>;
  on(type: 'status', handler: (status: Status) => void): BidirectionalStream<ReqT, ResT>;
}

export class TwitchRouletteServiceClient {
  readonly serviceHost: string;

  constructor(serviceHost: string, options?: grpc.RpcOptions);
  getRandomStream(
    requestMessage: twitchroulette_v1_twitchroulette_pb.GetRandomStreamRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: twitchroulette_v1_twitchroulette_pb.GetRandomStreamResponse|null) => void
  ): UnaryResponse;
  getRandomStream(
    requestMessage: twitchroulette_v1_twitchroulette_pb.GetRandomStreamRequest,
    callback: (error: ServiceError|null, responseMessage: twitchroulette_v1_twitchroulette_pb.GetRandomStreamResponse|null) => void
  ): UnaryResponse;
  getStreamTags(
    requestMessage: twitchroulette_v1_twitchroulette_pb.GetStreamTagsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: twitchroulette_v1_twitchroulette_pb.GetStreamTagsResponse|null) => void
  ): UnaryResponse;
  getStreamTags(
    requestMessage: twitchroulette_v1_twitchroulette_pb.GetStreamTagsRequest,
    callback: (error: ServiceError|null, responseMessage: twitchroulette_v1_twitchroulette_pb.GetStreamTagsResponse|null) => void
  ): UnaryResponse;
  getStreamGames(
    requestMessage: twitchroulette_v1_twitchroulette_pb.GetStreamGamesRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: twitchroulette_v1_twitchroulette_pb.GetStreamGamesResponse|null) => void
  ): UnaryResponse;
  getStreamGames(
    requestMessage: twitchroulette_v1_twitchroulette_pb.GetStreamGamesRequest,
    callback: (error: ServiceError|null, responseMessage: twitchroulette_v1_twitchroulette_pb.GetStreamGamesResponse|null) => void
  ): UnaryResponse;
}

