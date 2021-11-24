// package: twitchroulette.v1
// file: twitchroulette/v1/twitchroulette.proto

import * as jspb from "google-protobuf";

export class StreamTag extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getName(): string;
  setName(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StreamTag.AsObject;
  static toObject(includeInstance: boolean, msg: StreamTag): StreamTag.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StreamTag, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StreamTag;
  static deserializeBinaryFromReader(message: StreamTag, reader: jspb.BinaryReader): StreamTag;
}

export namespace StreamTag {
  export type AsObject = {
    id: string,
    name: string,
  }
}

export class StreamGame extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getImageUrl(): string;
  setImageUrl(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StreamGame.AsObject;
  static toObject(includeInstance: boolean, msg: StreamGame): StreamGame.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StreamGame, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StreamGame;
  static deserializeBinaryFromReader(message: StreamGame, reader: jspb.BinaryReader): StreamGame;
}

export namespace StreamGame {
  export type AsObject = {
    id: string,
    name: string,
    imageUrl: string,
  }
}

export class StreamUser extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getDisplayName(): string;
  setDisplayName(value: string): void;

  getLoginName(): string;
  setLoginName(value: string): void;

  getImageUrl(): string;
  setImageUrl(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StreamUser.AsObject;
  static toObject(includeInstance: boolean, msg: StreamUser): StreamUser.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StreamUser, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StreamUser;
  static deserializeBinaryFromReader(message: StreamUser, reader: jspb.BinaryReader): StreamUser;
}

export namespace StreamUser {
  export type AsObject = {
    id: string,
    displayName: string,
    loginName: string,
    imageUrl: string,
  }
}

export class Stream extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  hasUser(): boolean;
  clearUser(): void;
  getUser(): StreamUser | undefined;
  setUser(value?: StreamUser): void;

  hasGame(): boolean;
  clearGame(): void;
  getGame(): StreamGame | undefined;
  setGame(value?: StreamGame): void;

  getTitle(): string;
  setTitle(value: string): void;

  clearTagsList(): void;
  getTagsList(): Array<StreamTag>;
  setTagsList(value: Array<StreamTag>): void;
  addTags(value?: StreamTag, index?: number): StreamTag;

  getOnline(): boolean;
  setOnline(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Stream.AsObject;
  static toObject(includeInstance: boolean, msg: Stream): Stream.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Stream, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Stream;
  static deserializeBinaryFromReader(message: Stream, reader: jspb.BinaryReader): Stream;
}

export namespace Stream {
  export type AsObject = {
    id: string,
    user?: StreamUser.AsObject,
    game?: StreamGame.AsObject,
    title: string,
    tagsList: Array<StreamTag.AsObject>,
    online: boolean,
  }
}

export class GetRandomStreamRequest extends jspb.Message {
  clearIncludedTagIdsList(): void;
  getIncludedTagIdsList(): Array<string>;
  setIncludedTagIdsList(value: Array<string>): void;
  addIncludedTagIds(value: string, index?: number): string;

  clearExcludedTagIdsList(): void;
  getExcludedTagIdsList(): Array<string>;
  setExcludedTagIdsList(value: Array<string>): void;
  addExcludedTagIds(value: string, index?: number): string;

  clearIncludedGameIdsList(): void;
  getIncludedGameIdsList(): Array<string>;
  setIncludedGameIdsList(value: Array<string>): void;
  addIncludedGameIds(value: string, index?: number): string;

  clearExcludedGameIdsList(): void;
  getExcludedGameIdsList(): Array<string>;
  setExcludedGameIdsList(value: Array<string>): void;
  addExcludedGameIds(value: string, index?: number): string;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetRandomStreamRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetRandomStreamRequest): GetRandomStreamRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetRandomStreamRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetRandomStreamRequest;
  static deserializeBinaryFromReader(message: GetRandomStreamRequest, reader: jspb.BinaryReader): GetRandomStreamRequest;
}

export namespace GetRandomStreamRequest {
  export type AsObject = {
    includedTagIdsList: Array<string>,
    excludedTagIdsList: Array<string>,
    includedGameIdsList: Array<string>,
    excludedGameIdsList: Array<string>,
  }
}

export class GetRandomStreamResponse extends jspb.Message {
  hasStream(): boolean;
  clearStream(): void;
  getStream(): Stream | undefined;
  setStream(value?: Stream): void;

  getFilterId(): string;
  setFilterId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetRandomStreamResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetRandomStreamResponse): GetRandomStreamResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetRandomStreamResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetRandomStreamResponse;
  static deserializeBinaryFromReader(message: GetRandomStreamResponse, reader: jspb.BinaryReader): GetRandomStreamResponse;
}

export namespace GetRandomStreamResponse {
  export type AsObject = {
    stream?: Stream.AsObject,
    filterId: string,
  }
}

export class GetStreamTagsRequest extends jspb.Message {
  getParent(): string;
  setParent(value: string): void;

  getPageToken(): string;
  setPageToken(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetStreamTagsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetStreamTagsRequest): GetStreamTagsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetStreamTagsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetStreamTagsRequest;
  static deserializeBinaryFromReader(message: GetStreamTagsRequest, reader: jspb.BinaryReader): GetStreamTagsRequest;
}

export namespace GetStreamTagsRequest {
  export type AsObject = {
    parent: string,
    pageToken: string,
  }
}

export class GetStreamTagsResponse extends jspb.Message {
  clearTagsList(): void;
  getTagsList(): Array<StreamTag>;
  setTagsList(value: Array<StreamTag>): void;
  addTags(value?: StreamTag, index?: number): StreamTag;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetStreamTagsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetStreamTagsResponse): GetStreamTagsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetStreamTagsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetStreamTagsResponse;
  static deserializeBinaryFromReader(message: GetStreamTagsResponse, reader: jspb.BinaryReader): GetStreamTagsResponse;
}

export namespace GetStreamTagsResponse {
  export type AsObject = {
    tagsList: Array<StreamTag.AsObject>,
  }
}

export class GetStreamGamesRequest extends jspb.Message {
  getParent(): string;
  setParent(value: string): void;

  getPageToken(): string;
  setPageToken(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetStreamGamesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetStreamGamesRequest): GetStreamGamesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetStreamGamesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetStreamGamesRequest;
  static deserializeBinaryFromReader(message: GetStreamGamesRequest, reader: jspb.BinaryReader): GetStreamGamesRequest;
}

export namespace GetStreamGamesRequest {
  export type AsObject = {
    parent: string,
    pageToken: string,
  }
}

export class GetStreamGamesResponse extends jspb.Message {
  clearGamesList(): void;
  getGamesList(): Array<StreamGame>;
  setGamesList(value: Array<StreamGame>): void;
  addGames(value?: StreamGame, index?: number): StreamGame;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetStreamGamesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetStreamGamesResponse): GetStreamGamesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetStreamGamesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetStreamGamesResponse;
  static deserializeBinaryFromReader(message: GetStreamGamesResponse, reader: jspb.BinaryReader): GetStreamGamesResponse;
}

export namespace GetStreamGamesResponse {
  export type AsObject = {
    gamesList: Array<StreamGame.AsObject>,
  }
}

