#pragma once
#include <iostream>
#include <vector>
#include <fstream>
#include <assert.h>
#include "Neuron.h"
using namespace std;
class PoolingLayer {
private:
	vector<vector<Neuron>> pooling_n;
	unsigned fY, fX;
public:
	PoolingLayer(unsigned filX, unsigned filY, unsigned inputX, unsigned inputY);
	void feedForward(vector<vector<Neuron>> &cL);
	void backProp(vector<Neuron> &fL);
	void getResultVals(vector<double> &resultVals);
	void getLayer(vector<vector<Neuron>> &layer);
	void getPoolingFilter(unsigned &x, unsigned &y) { x = fX; y = fY; }
};
PoolingLayer::PoolingLayer(unsigned filX, unsigned filY, unsigned inputX, unsigned inputY) {
	fY = filY;
	fX = filX;
	int s = 0;
	for (int nY = 0; nY < ceil(double(inputY) / double(fY)); nY++) {
		pooling_n.push_back({});
		for (int nX = 0; nX < ceil(double(inputX) / double(fX)); nX++) {
			pooling_n.back().push_back(Neuron());
			s++;
		}
	}
	//cout << s << endl;
	//cout << "P: (y,x)" << pooling_n[0].size() << " " << pooling_n[0][0].size() << endl;
}
void PoolingLayer::getLayer(vector<vector<Neuron>> &layer) {
	layer.clear();
	for (int y = 0; y < pooling_n.size(); y++) {
		layer.push_back({});
		for (int x = 0; x < pooling_n[y].size(); x++) {
			layer.back().push_back(pooling_n[y][x]);
		}
	}
}
void PoolingLayer::feedForward(vector<vector<Neuron>> &cL) {
	//assert(cL.size() == pooling_n.size());
	//cout << "e" << endl;
	int x = 0;
	int y = 0;
	for (int pY = 0; pY < pooling_n.size(); pY++) {
		for (int pX = 0; pX < pooling_n[pY].size(); pX++) {
			double max = -1.0;
			int posX = -1;
			int posY = -1;
			for (int cY = y; cY < y + fY; cY++) {
				for (int cX = x; cX < x + fX; cX++) {
					if (cY < cL.size() && cX < cL[0].size() && cL[cY][cX].getOutputVal() > max) {
						max = cL[cY][cX].getOutputVal();
						posX = cX;
						posY = cY;
					}
				}
			}
			//cout << posX << " " << posY << endl;
			assert(posX != -1 && posY != -1);
			pooling_n[pY][pX].setOutputVal(cL[posY][posX].getOutputVal());
			pooling_n[pY][pX].setConvCoord(posX, posY);
			x++;
		}
		x = 0;
		y++;
	}
}
void PoolingLayer::backProp(vector<Neuron> &fL) {
	unsigned outputSize = 0;
	outputSize += (pooling_n.size() * pooling_n[0].size());
	assert(fL.size() == outputSize);
	unsigned fLPos = 0;
	for (int pY = 0; pY < pooling_n.size(); pY++) {
		for (int pX = 0; pX < pooling_n[pY].size(); pX++) {
			pooling_n[pY][pX].setGradient(fL[fLPos].getGradient());
			fLPos++;
		}
	}
}
void PoolingLayer::getResultVals(vector<double> &resultVals) {
	for (int y = 0; y < pooling_n.size(); y++) {
		//resultVals.back().push_back({});
		for (int x = 0; x < pooling_n[y].size(); x++) {
			resultVals.push_back(pooling_n[y][x].getOutputVal());
		}
	}
}