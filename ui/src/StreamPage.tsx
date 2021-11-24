import { FunctionComponent } from 'preact';
import { useRef, useEffect, useState } from 'preact/hooks';
import { useLocation } from 'wouter-preact';
import { useKeyPress } from './hooks';

import { getRandomStream } from './service';

declare global {
  interface Window {
    Twitch: any;
  }
}

const StreamPage: FunctionComponent<{ id: string }> = ({ id }) => {
  const [_, setLocation] = useLocation();
  const [embed, setEmbed] = useState<any>(null);
  const rPressed: boolean = useKeyPress('r');

  const containerRef = useRef<HTMLDivElement>(null);

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

  useEffect(() => {
    async function handleKeyPress() {
      await spin('https://api.twitchroulette.net', [], []);
    }

    handleKeyPress();
  }, [rPressed]);

  useEffect(() => {
    if (!embed) {
      const newEmbed = new window.Twitch.Embed(containerRef.current, {
        width: '100%',
        height: '97.5%',
        channel: id,
      });

      setEmbed(newEmbed);
    } else {
      const player = embed.getPlayer();
      player.setChannel(id);
    }
  }, [containerRef, id]);

  return (
    <div className="h-full w-full overflow-y-hidden">
      <div className="h-8 bg-gray-800 flex flex-row w-full text-center content-center justify-center text-gray-400">
        <p
          onClick={() => setLocation('/')}
          className="p-0.5 mr-2 cursor-pointer"
        >
          [ home ]
        </p>
        <p
          onClick={() =>
            (async () => await spin('https://api.twitchroulette.net', [], []))()
          }
          className="p-0.5 cursor-pointer"
        >
          [ respin (r) ]
        </p>
      </div>
      <div className="h-full" ref={containerRef}></div>
    </div>
  );
};

export default StreamPage;
