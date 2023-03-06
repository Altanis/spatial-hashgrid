#include <iostream>

#include <unordered_map>
#include <unordered_set>

#include <vector>
#include <utility>
#include <cmath>

const int cell_size = 256;
const int cell_size_log2 = std::log2(cell_size);

struct PairHash {
    std::size_t operator()(const std::pair<int, int>& p) const {
        return p.first | (((uint64_t) p.second) << 32);
    }
};

class SpatialHashGrid {
	public:
		void insert(int x, int y, int width, int height, int id) {
			int x1 = x >> cell_size_log2;
			int y1 = y >> cell_size_log2;
			int x2 = (x + width) >> cell_size_log2;
			int y2 = (y + height) >> cell_size_log2;

			for (int i = x1; i <= x2; i++) {
				for (int j = y1; j <= y2; j++) {
					grid_[std::make_pair(i, j)].push_back(std::make_pair(x, y));
					dimensions_[std::make_pair(x, y)] = std::make_pair(width, height);
					ids_[std::make_pair(x, y)].insert(id);
				}
			}
		}

		std::vector<int> query(int x, int y, int width, int height) {
			std::unordered_set<int> result;

			int x1 = x >> cell_size_log2;
			int y1 = y >> cell_size_log2;
			int x2 = (x + width) >> cell_size_log2;
			int y2 = (y + height) >> cell_size_log2;

			for (int i = x1; i <= x2; i++) {
				for (int j = y1; j <= y2; j++) {
					for (auto& coords : grid_[std::make_pair(i, j)]) {
						int entity_x = coords.first;
						int entity_y = coords.second;
						int entity_width = dimensions_[coords].first;
						int entity_height = dimensions_[coords].second;
						if (collides_with(entity_x, entity_y, entity_width, entity_height, x, y, width, height)) {
							for (auto id : ids_[std::make_pair(entity_x, entity_y)]) {
								result.insert(id);
							}
						}
					}
				}
			}

			return std::vector<int>(result.begin(), result.end());
		}

		void clear() {
			grid_.clear();
			dimensions_.clear();
			ids_.clear();
		}

	private:
		bool collides_with(int entity_x, int entity_y, int entity_width, int entity_height, int x, int y, int width, int height) {
			return x < entity_x + entity_width &&
				entity_x < x + width &&
				y < entity_y + entity_height &&
				entity_y < y + height;
		}

		std::unordered_map<std::pair<int, int>, std::vector<std::pair<int, int>>, PairHash> grid_;
		std::unordered_map<std::pair<int, int>, std::pair<int, int>, PairHash> dimensions_;
		std::unordered_map<std::pair<int, int>, std::unordered_set<int>, PairHash> ids_;
};

int main() {
	/** Instantiate grid. */
	SpatialHashGrid* shg = new SpatialHashGrid();

	/** Insert entities. */
	shg->insert(0, 0, 10, 10, 1);
	shg->insert(10, 10, 10, 10, 2);
	shg->insert(1, 1, 100, 100, 3);

	/** Query entities. */
	auto result = shg->query(0, 0, 10, 10);
	for (auto& id : result) {
		std::cout << id << std::endl;
	}

	return 0;
}