import { createContext, FunctionComponent } from 'preact';
import { useState } from 'preact/hooks';

export interface FilterState {
  excludedTags: string[];
  excludedGames: string[];
}

export const FilterStateContext = createContext<{
  filters: FilterState;
  setFilters: (filters: FilterState) => void;
}>({
  filters: {
    excludedTags: [],
    excludedGames: [],
  },
  setFilters: () => {},
});

export const FilterStateProvider: FunctionComponent = ({ children }) => {
  const [filters, setFilters] = useState<FilterState>({
    excludedTags: [],
    excludedGames: [],
  });

  return (
    <FilterStateContext.Provider
      value={{
        filters,
        setFilters,
      }}
    >
      {children}
    </FilterStateContext.Provider>
  );
};
