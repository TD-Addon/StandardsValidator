'use strict';

const PREFIXES = [];
const LOCALS = new Set(require('../data/projects').reduce((all, p) => {
	if(p.local) {
		all.push(p.local.toLowerCase());
	}
	PREFIXES.push(p.prefix.toLowerCase());
	return all;
}, []));
const HIGH_RANK = 7;

function overridesVanilla(record) {
	if(record.result) {
		return /(^|\n)\s*;\s*SV:\s*vanilla\s+override\s*($|\n)/i.test(record.result);
	}
	return false;
}

function needsNoLore(record, topic, mode) {
	return !(record.data.dialogue_type === 'Greeting' && mode === 'TD')
		&& !(record.data?.speaker_rank >= HIGH_RANK) // High-level faction dialogue
		&& record.speaker_class?.toLowerCase() !== 'slave' // Slaves are generally NoLore
		&& topic.id !== 'Greeting 9' // Has greetings for NoLore at the top
		&& !overridesVanilla(record);
}

module.exports = {
	onInfo(record, topic, mode) {
		if(record.speaker_id === 'dialog placeholder') {
			return;
		}
		if(!record.text?.length) {
			if(!['Journal', 'Voice'].includes(record.data?.dialogue_type)) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'has no text');
			}
		} else {
			if(/[^\S\r\n]{2,}/.test(record.text)) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'contains double spaces');
			}
			if(record.text.includes(' - ')) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'contains a single hyphen');
			}
			if(record.text.includes('....')) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'contains an overlong ellipsis');
			}
			if(/[^.]\.{2}[^.?]/.test(record.text)) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'contains a short ellipsis');
			}
			if(/\s[.,:;?]($|\s)/.test(record.text)) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'contains punctuation preceded by whitespace');
			}
			if(/[,:;]{2,}/.test(record.text)) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'contains doubled up punctuation');
			}
		}
		if(record.speaker_id) {
			if(record.speaker_rank) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary race filter');
			}
			if(record.speaker_class) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary class filter');
			}
			if(record.speaker_faction) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary faction filter');
			}
			if(record.data?.speaker_sex && record.data.speaker_sex !== 'Any') {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary sex filter');
			}
			record.filters?.forEach(filter => {
				if(['Local', 'NotLocal'].includes(filter.filter_type)) {
					const variable = filter.id?.toLowerCase();
					if(['nolore', 't_local_nolore', 't_local_khajiit', 't_local_npc'].includes(variable)) {
						console.log(record.type, record.info_id, 'in topic', topic.id, 'has a', filter.id, 'filter');
					}
				} else if(filter.filter_type === 'NotFaction') {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary Not Faction filter');
				} else if(filter.filter_type === 'NotClass') {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary Not Class filter');
				} else if(filter.filter_type === 'NotRace') {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary Not Race filter');
				} else if(filter.filter_type === 'NotId') {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary Not ID filter');
				}
			});
		} else if(record.data?.dialogue_type === 'Voice') {
			const project = record.filters?.some(filter => {
				if(filter.filter_type === 'Local' && filter.id) {
					const id = filter.id.toLowerCase();
					return id === 't_local_npc' || id === 't_local_khajiit' || LOCALS.has(id) || PREFIXES.some(p => id.startsWith(p));
				}
				return false;
			});
			if(!project) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'does not have a known project specific local filter');
			}
		} else if(['Greeting', 'Topic', 'Persuasion'].includes(record.data?.dialogue_type)) {
			const isServiceRefusal = topic.id === 'Service Refusal';
			let project = false;
			let nolore = false;
			let vanillaNoLore = false;
			let choice = false;
			record.filters?.forEach(filter => {
				if(filter.filter_type === 'Local') {
					const id = filter.id?.toLowerCase();
					if(id === 't_local_nolore' || id === 'nolore') {
						console.log(record.type, record.info_id, 'in topic', topic.id, 'has a Local', filter.id, 'filter');
					} else if(!project || !nolore) {
						if(id === 't_local_npc' || id === 't_local_khajiit' || LOCALS.has(id)) {
							project = true;
						} else if(id && PREFIXES.some(p => id.startsWith(p))) {
							project = true;
							nolore = true;
						}
					}
					if(id === 't_local_npc' && (filter.filter_comparison !== 'Equal' || filter.value?.Integer !== 0) || id === 't_local_khajiit' && (filter.filter_comparison !== 'Equal' || filter.value?.Integer !== 1)) {
						console.log(record.type, record.info_id, 'in topic', topic.id, 'has a Local', filter.id, filter.filter_comparison, filter.value?.Integer, 'filter');
					}
				} else if(filter.filter_type === 'NotLocal') {
					const id = filter.id?.toLowerCase();
					if(id === 't_local_nolore') {
						nolore = true;
						if(filter.filter_comparison !== 'Equal' || filter.value?.Integer !== 0) {
							console.log(record.type, record.info_id, 'in topic', topic.id, 'has a Not Local', filter.id, filter.filter_comparison, filter.value?.Integer, 'filter');
						}
					} else if(id === 'nolore' && filter.filter_comparison === 'Equal' && filter.value?.Integer === 0) {
						vanillaNoLore = true;
					} else if(id === 't_local_npc' || id === 't_local_khajiit' && (filter.filter_comparison !== 'Equal' || filter.value?.Integer !== 1)) {
						console.log(record.type, record.info_id, 'in topic', topic.id, 'has a Not Local', filter.id, filter.filter_comparison, filter.value?.Integer, 'filter');
					}
				} else if(filter.filter_type === 'Function' && filter.filter_function === 'Choice') {
					choice = true;
				}
			});
			if(!project && record.speaker_faction) {
				const faction = record.speaker_faction.toLowerCase();
				project = PREFIXES.some(p => faction.startsWith(p));
			}
			if(vanillaNoLore) {
				if(project) {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has a Not Local NoLore filter');
				} else {
					return;
				}
			}
			if(!project && !(isServiceRefusal && mode === 'TD') && !choice && !overridesVanilla(record)) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'does not have a known project specific local filter');
			}
			if(!nolore && !isServiceRefusal && !choice && needsNoLore(record, topic, mode)) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'does not have a T_Local_NoLore filter');
			}
		}
	}
};
