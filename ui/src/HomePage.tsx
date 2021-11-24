import { useLocation } from 'wouter-preact';

import { getRandomStream } from './service';

const StreamPage = () => {
  const [_, setLocation] = useLocation();

  const spin = async (
    host: string,
    excludedGameIds: string[],
    excludedTagIds: string[],
  ) => {
    const result = await getRandomStream(host, excludedGameIds, excludedTagIds);
    const streamer = result.getUser()?.getLoginName()?.toString();

    if (streamer) {
      setLocation(`/${streamer}`);
    }
  };

  return (
    <div className="flex flex-col justify-center sm:px-8 lg:px-12 mt-36">
      <div className="sm:mx-auto sm:w-full sm:max-w-md">
        <h2 className="mt-6 text-center text-6xl font-extrabold text-gray-100">
          Twitchroulette
        </h2>
        <p className="mt-2 text-center text-sm text-gray-100">
          Find a new friend among Twitch streamers who need you the most
        </p>
        <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
          <div>
            <button
              className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              onClick={() =>
                (async () =>
                  await spin('https://api.twitchroulette.net', [], []))()
              }
            >
              Spin
            </button>
          </div>
          {/* <p className="mt-6 text-center text-sm text-gray-100 flex flex-row content-center justify-center">
            <p className="mr-2">[ +exclude game ]</p>
            <p>[ +exclude tag ]</p>
          </p> */}
        </div>
      </div>
    </div>
  );
};

export default StreamPage;
