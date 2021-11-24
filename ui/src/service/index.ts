import { grpc } from '@improbable-eng/grpc-web';

import { TwitchRouletteService } from './twitchroulette/v1/twitchroulette_pb_service';
import {
  GetRandomStreamRequest,
  GetRandomStreamResponse,
  Stream,
} from './twitchroulette/v1/twitchroulette_pb';

export const getRandomStream = async (
  host: string,
  excludedGameIds: string[],
  excludedTagIds: string[],
): Promise<Stream> => {
  const request = new GetRandomStreamRequest();
  request.setExcludedGameIdsList(excludedGameIds);
  request.setExcludedTagIdsList(excludedTagIds);

  return new Promise((resolve, reject) => {
    grpc.unary(TwitchRouletteService.GetRandomStream, {
      request,
      host,
      onEnd: (res) => {
        const { status, statusMessage, message } = res;

        if (status === grpc.Code.OK && message) {
          const response = message as GetRandomStreamResponse;
          const stream = response.getStream();

          if (stream) {
            resolve(stream);
          } else {
            reject(statusMessage);
          }
        } else {
          reject(statusMessage);
        }
      },
    });
  });
};
