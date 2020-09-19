#pragma once
#include <iostream>
#include <vector>
#include <fstream>
#include <assert.h>
#include "Neuron.h"
using namespace std;
class CLayer {
private:
	vector<vector<vector<Neuron>>> conv_n;
	vector<vector<double>> filterW;
	double bias;
public:
	CLayer(unsigned fX, unsigned fY, unsigned inputX, unsigned inputY);
	void feedForward(vector<vector<double>> &inputVals);
	void backProp(vector<vector<Neuron>> &pL);
	void backProp_hiddenLayer(vector<vector<Neuron>> &cL);
	void backProp(vector<Neuron> &fL);
	void getOutputLayers(vector<vector<Neuron>> &cOut);
	void getResultVals(vector<double> &resultVals);
	void getResultVals(vector<vector<double>> &resultVals);
	void setWeights(vector<double> &w);
	void getInputSize(unsigned &x, unsigned &y) { x = conv_n[0][0].size(); y = conv_n[0].size(); }
	void getFilterSize(unsigned &x, unsigned &y) { x = filterW[0].size(); y = filterW.size(); }
	void getWeights(vector<double> &w);
};
CLayer::CLayer(unsigned fX, unsigned fY, unsigned inputX, unsigned inputY) {
	conv_n.push_back({}); //new neuron layer in conv layer (input layer)
	for (int nY = 0; nY < inputY; nY++) { //add neurons in input layer
		conv_n.back().push_back({});
		for (int nX = 0; nX < inputX; nX++) {
			conv_n.back().back().push_back(Neuron());
		}
	}
	conv_n.push_back({}); //new neuron layer in conv layer (output layer)
	int pX = 0;
	int pY = 0;
	for (int nY = 0; nY < (inputY - fY + 1); nY++) {
		conv_n.back().push_back({});
		for (int nX = 0; nX < (inputX - fX + 1); nX++) {
			conv_n.back().back().push_back(Neuron(pX, pY, 1));
			pX++;
		}
		pX = 0;
		pY++;
	}
	for (int wY = 0; wY < fY; wY++) {
		filterW.push_back({});
		for (int wX = 0; wX < fX; wX++) {
			filterW.back().push_back(rand() / double(RAND_MAX));
		}
	}
	bias = (rand() / double(RAND_MAX));
}
void CLayer::getWeights(vector<double> &w) {
	w.clear();
	for (int y = 0; y < filterW.size(); y++) {
		for (int x = 0; x < filterW[y].size(); x++) {
			w.push_back(filterW[y][x]);
		}
	}
	w.push_back(bias);
}
void CLayer::setWeights(vector<double> &w) {
	unsigned loc = 0;
	for (int y = 0; y < filterW.size(); y++) {
		for (int x = 0; x < filterW[y].size(); x++) {
			filterW[y][x] = w[loc];
			loc++;
		}
	}
	bias = w[loc];
}
void CLayer::feedForward(vector<vector<double>> &inputVals) {
	assert(inputVals.size() == conv_n[0].size() && inputVals[0].size() == conv_n[0][0].size());
	for (int nY = 0; nY < conv_n[0].size(); nY++) {
		for (int nX = 0; nX < conv_n[0][nY].size(); nX++) {
			conv_n[0][nY][nX].setOutputVal(inputVals[nY][nX]);
		}
	}
	for (int nY = 0; nY < conv_n[1].size(); nY++) {
		for (int nX = 0; nX < conv_n[1][nY].size(); nX++) {
			conv_n[1][nY][nX].feedForward(conv_n[0], filterW, bias);
		}
	}
}
void CLayer::backProp_hiddenLayer(vector<vector<Neuron>> &cL) {
	//assert(pL.size() == conv_n[1].size() && pL[0].size()==conv_n[1][0].size());
	int cLSize = cL.size() * cL[0].size();
	for (int Y = 0; Y < cL.size(); Y++) {
		for (int X = 0; X < cL[Y].size(); X++) {
			conv_n[1][Y][X].setGradient(cL[Y][X].getGradient());
			conv_n[1][Y][X].updatePrevLayerGradient(conv_n[0], filterW);
		}
	}
	for (int Y = 0; Y < cL.size(); Y++) {
		for (int X = 0; X < cL[Y].size(); X++) {
			conv_n[1][Y][X].updateInputWeights(conv_n[0], filterW, bias, cLSize);
		}
	}
}
void CLayer::backProp(vector<vector<Neuron>> &pL) {
	//assert(pL.size() == conv_n[1].size() && pL[0].size()==conv_n[1][0].size());
	int pLSize = pL.size() * pL[0].size();
	for (int pY = 0; pY < pL.size(); pY++) {
		for (int pX = 0; pX < pL[pY].size(); pX++) {
			unsigned cX = pL[pY][pX].getConvX(); //get conv neuron that is max
			unsigned cY = pL[pY][pX].getConvY();
			conv_n[1][cY][cX].setGradient(pL[pY][pX].getGradient());
			conv_n[1][cY][cX].updatePrevLayerGradient(conv_n[0], filterW);
		}
	}
	for (int pY = 0; pY < pL.size(); pY++) {
		for (int pX = 0; pX < pL[pY].size(); pX++) {
			unsigned cX = pL[pY][pX].getConvX(); //get conv neuron that is max
			unsigned cY = pL[pY][pX].getConvY();
			conv_n[1][cY][cX].updateInputWeights(conv_n[0], filterW, bias, pLSize);
		}
	}
}
void CLayer::backProp(vector<Neuron> &fL) {
	unsigned fLPos = 0;
	for (int nY = 0; nY < conv_n[1].size(); nY++) {
		for (int nX = 0; nX < conv_n[1][nY].size(); nX++) {
			conv_n[1][nY][nX].setGradient(fL[fLPos].getGradient());
			conv_n[1][nY][nX].updateInputWeights(conv_n[0], filterW, bias, 1);
			fLPos++;
		}
	}
}
void CLayer::getOutputLayers(vector<vector<Neuron>> &cOut) {
	cOut.clear();
	for (int y = 0; y < conv_n[1].size(); y++) {
		cOut.push_back({});
		for (int x = 0; x < conv_n[1][y].size(); x++) {
			cOut.back().push_back(conv_n[1][y][x]);
		}
	}
}
void CLayer::getResultVals(vector<double> &resultVals) {
	resultVals.clear();
	for (int y = 0; y < conv_n[1].size(); y++) {
		for (int x = 0; x < conv_n[1][y].size(); x++) {
			resultVals.push_back(conv_n[1][y][x].getOutputVal());
		}
	}
}
void CLayer::getResultVals(vector<vector<double>> &resultVals) {
	resultVals.clear();
	for (int y = 0; y < conv_n[1].size(); y++) {
		resultVals.push_back({});
		for (int x = 0; x < conv_n[1][y].size(); x++) {
			resultVals.back().push_back(conv_n[1][y][x].getOutputVal());
		}
	}
}