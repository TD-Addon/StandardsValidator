'use strict';

const { isDead, isInterior, getCellGrid, getCellName } = require('./util');

const CLASSES = new Set(require('../data/travel').map(c => c.toLowerCase()));
const CARAVANERS = {};
const CELLS = {};

function matches(dest, interior, id, grid) {
	if(interior) {
		return dest.cell?.toLowerCase() === id;
	}
	const [x, y] = dest.translation;
	const g = getCellGrid(x, y);
	if(g[0] === grid[0] && g[1] === grid[1]) {
		return true;
	}
	const dX = Math.abs(g[0] - grid[0]);
	const dY = Math.abs(g[1] - grid[1]);
	return dX <= 1 && dY <= 1;
}

function getTownName(cellId) {
	if(cellId) {
		const [base] = cellId.split(',');
		return base;
	}
}

function getDestinationName(dest) {
	if(dest.cell) {
		return [dest.cell, getTownName(dest.cell)];
	}
	const grid = getCellGrid(...dest.translation);
	const cell = CELLS[grid.join(',')];
	if(cell) {
		return [getCellName(cell), getTownName(cell.id)];
	}
	return [grid];
}

module.exports = {
	onRecord(record, mode, id) {
		if(record.type === 'Cell' && !isInterior(record)) {
			CELLS[record.data.grid.join(',')] = record;
		}
		if(!['Creature', 'Npc'].includes(record.type) || isDead(record)) {
			return;
		}
		if(record.type === 'Npc' && CLASSES.has(record.class?.toLowerCase()) && !record.travel_destinations?.length) {
			console.log(record.type, record.id, 'has class', record.class, 'but does not offer travel services');
		}
		if(record.travel_destinations?.length) {
			CARAVANERS[id] = { record, cells: [], destination: [] };
		}
	},
	onCellRef(record, reference, id) {
		if(id in CARAVANERS) {
			const interior = isInterior(record);
			const cellId = interior && record.id.toLowerCase();
			const grid = !interior && record.data.grid;
			CARAVANERS[id].cells.push({
				record,
				matches: dest => matches(dest, interior, cellId, grid)
			});
		}
	},
	onInfo(record, topic) {
		if(record.speaker_id && /^destination$/i.test(topic.id)) {
			const id = record.speaker_id.toLowerCase();
			if(id in CARAVANERS) {
				CARAVANERS[id].destination.push(record);
			}
		}
	},
	onEnd() {
		const caravaners = Object.values(CARAVANERS);
		caravaners.forEach(({ record, cells, destination }) => {
			if(!destination.length) {
				console.log(record.type, record.id, 'offers travel services but does not have a reply to the destination topic');
			}
			cells.forEach(cell => {
				const counterparts = caravaners.filter(c => c.record.travel_destinations.some(dest => cell.matches(dest)));
				record.travel_destinations.forEach(dest => {
					const returnServices = counterparts.filter(c => c.cells.some(l => l.matches(dest)));
					const [destName, town] = getDestinationName(dest);
					if(!returnServices.length) {
						console.log(record.type, record.id, 'in', getCellName(cell.record), 'offers travel to', destName, 'but there is no return travel there');
					} else if(record.class && !returnServices.some(counterpart => !counterpart.record.class || record.class === counterpart.record.class)) {
						console.log(record.type, record.id, 'in', getCellName(cell.record), 'offers', record.class, 'travel to', destName, 'but there is no corresponding return travel');
					}
					if(town && destination.length && !destination.some(info => info.text.includes(town))) {
						console.log(record.type, record.id, 'does not mention', town, 'in their destination response');
					}
				});
			});
		});
	}
};
